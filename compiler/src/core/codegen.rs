use std::collections::{HashSet, HashMap};

use super::*;
use crate::symbol::*;
use super::reg_alloc::{AllocScan, RegAlloc};
use super::visitor::*;

/*
    bytecode generation
*/

pub struct CodeGen {
    blocks: HashMap<Symbol,ByteCodeBlock>,
    current: Vec<ByteCode>,
}

impl CodeGen {
    pub fn new() -> CodeGen {
        CodeGen {
            blocks: HashMap::new(),
            current: Vec::new(),
        }
    }
    pub fn run(expr: Core) -> HashMap<Symbol,ByteCodeBlock> {
        let mut ctx = CodeGen::new();
        ctx.walk_cexpr(expr);
        let old_current = ctx.current.drain(0..).collect();

        let main = newvar("___main___");
        let block = ByteCodeBlock {
            func: main,
            args: 0,
            body: old_current,
        };

        ctx.blocks.insert(main, block);
        ctx.blocks
    }
}

/*
    return a vec with "need" register,
    these registers have index larger than "least"
    and not contained in "used"
*/



impl VisitorTopDown for CodeGen {

    fn visit_app(&mut self, expr: CoreApp) -> Core {
        let CoreApp { func, args } = expr;

        let used: Vec<usize> = args.iter()
            .filter_map(|arg| {
                if let Atom::Var(Symbol::Reg(n)) = arg
                { Some(*n) } else { None }
            })
            .collect();
        
        let mut temp_reg: Vec<usize> = (args.len()..)
            .filter(|n| !used.contains(n))
            .take(2)
            .collect();

        let func_moved: Option<usize> =
            if let Atom::Var(Symbol::Reg(n)) = func {
                if n < args.len() {
                    let temp = temp_reg.pop().unwrap();
                    self.current.push(ByteCode::Move(func, temp));
                    Some(temp)
                } else {
                    None
                }
            } else {
                None
            };

        let mut map: HashMap<usize,usize> = args.iter()
            .copied().enumerate()
            .filter_map(|(j,arg)| {
                if let Atom::Var(Symbol::Reg(i)) = arg {
                    Some((i,j)) // ri -> rj
                } else {
                    None
                }
            })
            .collect();

        loop {
            let option_ij = map.drain().take(1).nth(0);

            if let Some((i,j)) = option_ij {
                let mut chain = vec![i,j];
                loop {

                    if let Some(k) = map.get(chain.last().unwrap()) {
                        
                        if *k == chain[0] {
                            // circle!
                            let temp = temp_reg.pop().unwrap();
                            let last = *chain.last().unwrap();

                            self.current.push(ByteCode::Move(
                                Atom::Var(Symbol::Reg(last)),
                                temp,
                            ));
                            for i in (chain.len() - 1)..0 {
                                let j = i + 1;
                                self.current.push(ByteCode::Move(
                                    Atom::Var(Symbol::Reg(chain[i])),
                                    chain[j],
                                ));
                                map.remove(&i);
                            }
                            self.current.push(ByteCode::Move(
                                Atom::Var(Symbol::Reg(temp)),
                                last,
                            ));
                            break;
                        } else {
                            chain.push(*k);
                            continue;
                        }
                    } else {
                        for i in (0..chain.len() - 1).rev() {
                            let j = i + 1;
                            self.current.push(ByteCode::Move(
                                Atom::Var(Symbol::Reg(chain[i])),
                                chain[j],
                            ));
                            map.remove(&i);
                        }
                        break;
                    }
                }
                
                
            } else {
                assert!(map.is_empty());
                break;
            }
        }

        // move all literal arg to the curresponding register
        args.iter()
            .copied().enumerate()
            .for_each(|(j,arg)| {
                if let Atom::Var(Symbol::Reg(_)) = arg {
                    // do nothing
                } else {
                    self.current.push(ByteCode::Move(arg, j));
                }
            });

        if let Some(n) = func_moved {
            self.current.push(ByteCode::Jump(
                Atom::Var(Symbol::Reg(n))));
        } else {
            self.current.push(ByteCode::Jump(func));
        }

        Core::App(CoreApp { func, args })
    }

    fn visit_halt(&mut self, arg: Atom) -> Core {
        self.current.push(ByteCode::Halt(arg));

        Core::Halt(arg)   
    }

    fn visit_opr(&mut self, expr: CoreOpr) -> Core {
        let CoreOpr { prim, args, bind, cont } = expr;
        let bind_reg = if let Symbol::Reg(n) = bind {
            n
        } else {
            panic!("not a register!");
        };
        
        match prim {
            Prim::IAdd => {
                self.current.push(
                    ByteCode::IAdd(args[0], args[1], bind_reg))
            }
            Prim::ISub => {
                self.current.push(
                    ByteCode::ISub(args[0], args[1], bind_reg))
            }
            Prim::IMul => {
                self.current.push(
                    ByteCode::IMul(args[0], args[1], bind_reg))
            }
            Prim::IDiv => {
                self.current.push(
                    ByteCode::IDiv(args[0], args[1], bind_reg))
            }
            Prim::INeg => {
                self.current.push(
                    ByteCode::INeg(args[0], bind_reg))
            }
            Prim::BNot => {
                self.current.push(
                    ByteCode::BNot(args[0], bind_reg))
            }
        }

        let cont = Box::new(self.walk_cexpr(*cont));

        Core::Opr(CoreOpr { prim, args, bind, cont })
    }

    fn visit_decl(&mut self, decl: CoreDecl) -> CoreDecl {
        let CoreDecl { func, args, body } = decl;

        let body = Box::new(self.walk_cexpr(*body));
        
        let old_current = self.current.drain(0..).collect();
        let block = ByteCodeBlock {
            func: func,
            args: args.len(),
            body: old_current,
        };
        
        self.blocks.insert(func, block);

        CoreDecl { func, args, body }
    }

    fn visit_let(&mut self, expr: CoreLet) -> Core {
        let CoreLet { decl, cont } = expr;
        let old_current: Vec<ByteCode> = 
            self.current.drain(0..).collect();

        let decl = self.visit_decl(decl);

        old_current.into_iter()
            .for_each(|code| self.current.push(code));

        let cont = Box::new(self.walk_cexpr(*cont));
        Core::Let(CoreLet { decl, cont })
    }
}

#[test]
fn opt_test() {
    use crate::parser::*;
    let string = "
        (fn x y => (* (+ x 1) (- y 2))) 3 4
    ";
    let mut par = Parser::new(string);

    let res = parse_program(&mut par);
    if let Ok(res) = res {
        println!("\n{res}");
        let expr = super::cps_trans::cps_trans_top(&res);
        println!("\n{}", expr);
        let expr = AllocScan::run(expr);
        println!("\n{}", expr);
        let expr = RegAlloc::run(expr);
        println!("\n{}\n", expr);
        let blocks = CodeGen::run(expr);
        for (k,v) in blocks {
            println!("{}", v);
        }
    } else {
        par.print_err();
    }
}