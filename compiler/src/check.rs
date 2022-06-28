use crate::ast::*;
use crate::symbol::Symbol;
use crate::utils::*;

pub struct Checker {
    val_env: MultiSet<Symbol>,
    data_env: MultiSet<Symbol>,
    cons_env: MultiSet<Symbol>,
    type_env: MultiSet<Symbol>,
    thrown: bool,
    error: Vec<CheckError>,

}

#[derive(Clone, Debug, PartialEq)]
pub enum CheckError {
    ValNotBound(Symbol),
    DataNotBound(Symbol),
    ConsNotBound(Symbol),
    TypeNotBound(Symbol),
    ArgsNotUnique(Symbol),
    RealNumberInPattern,
    ErrorIn(Span, &'static str),
}

type CheckResult<T> = Result<T, ()>;
type CheckFunc<T> = fn(&mut Checker) -> CheckResult<T>;

impl Checker {
    pub fn new() -> Checker {
        Checker {
            val_env: MultiSet::new(),
            data_env: MultiSet::new(),
            cons_env: MultiSet::new(),
            type_env: MultiSet::new(),
            thrown: false,
            error: Vec::new(),
        }
    }

    pub fn catch_start(&mut self) -> bool {
        let old = self.thrown;
        self.thrown = false;
        old
    }

    pub fn catch_finish(&mut self, old: bool, err: CheckError) {
        if self.thrown {
            // keeps self.thrown = true;
            self.error.push(err);
        } else {
            self.thrown = old;
        }
    }

    pub fn throw_err(&mut self, err: CheckError) {
        self.thrown = true;
        self.error.push(err);
    }

    pub fn check_expr_var(&mut self, expr: &ExprVar) {
        let old = self.catch_start();

        if !self.val_env.contains(&expr.ident) {
            self.throw_err(CheckError::ValNotBound(expr.ident));
        }

        self.catch_finish(old,
            CheckError::ErrorIn(expr.span, "Variable"));
    }

    pub fn check_expr_lam(&mut self, expr: &ExprLam) {
        let old = self.catch_start();

        for arg in &expr.args {
            self.val_env.insert(*arg);
        }

        self.check_expr(&*expr.body);

        for arg in &expr.args {
            self.val_env.remove(arg);
        }

        self.catch_finish(old,
            CheckError::ErrorIn(expr.span, "Lambda"));
    }

    pub fn check_expr_app(&mut self, expr: &ExprApp) {
        let old = self.catch_start();

        self.check_expr(&*expr.func);

        for arg in &expr.args {
            self.check_expr(arg);
        }

        self.catch_finish(old,
            CheckError::ErrorIn(expr.span, "Application"));
    }

    pub fn check_expr_let(&mut self, expr: &ExprLet) {
        let old = self.catch_start();

        for decl in &expr.decls {
            self.enter_decl(decl);
        }
        self.check_expr(&*expr.body);

        for decl in &expr.decls {
            self.leave_decl(decl);
        }

        self.catch_finish(old,
            CheckError::ErrorIn(expr.span, "Let-Block"));
    }

    pub fn enter_decl(&mut self, decl: &Decl) {
        match decl {
            Decl::Val(decl) => {
                self.enter_val_decl(decl)
            }
            Decl::Data(decl) => {
                self.enter_data_decl(decl)
            }
            Decl::Type(decl) => {
                self.enter_type_decl(decl)
            }
        }
    }

    pub fn leave_decl(&mut self, decl: &Decl) {
        match decl {
            Decl::Val(decl) => {
                self.enter_val_decl(decl)
            }
            Decl::Data(decl) => {
                self.enter_data_decl(decl)
            }
            Decl::Type(decl) => {
                self.enter_type_decl(decl)
            }
        }
    }

    pub fn check_unique(&mut self, syms: &Vec<Symbol>) {
        let len = syms.len();

        for i in 0..len {
            for j in i..len {
                if syms[i] == syms[j] {
                    self.throw_err(CheckError::ArgsNotUnique(syms[i]));
                    return;
                }
            }
        }
    }

    pub fn enter_val_decl(&mut self, decl: &DeclVal) {
        let old = self.catch_start();
        
        self.val_env.insert(decl.name);
        self.check_unique(&decl.args);

        for arg in &decl.args {
            self.type_env.insert(*arg);
        }
        self.check_expr(&decl.body);
        for arg in &decl.args {
            self.type_env.remove(arg);
        }

        self.catch_finish(old,
            CheckError::ErrorIn(decl.span, "Value Decl"));
    }

    pub fn leave_val_decl(&mut self, decl: &DeclVal) {
        self.val_env.remove(&decl.name);
    }

    pub fn enter_data_decl(&mut self, decl: &DeclData) {
        let old = self.catch_start();

        self.type_env.insert(decl.name);
        self.check_unique(&decl.args);

        for arg in &decl.args {
            self.type_env.insert(*arg);
        }

        for var in &decl.vars {
            self.enter_varient(var);
        }
        
        for arg in &decl.args {
            self.type_env.remove(arg);
        }

        self.catch_finish(old,
            CheckError::ErrorIn(decl.span, "Data Decl"));
    }

    pub fn leave_data_decl(&mut self, decl: &DeclData) {
        self.type_env.remove(&decl.name);

        for var in &decl.vars {
            self.leave_varient(var);
        }
    }

    pub fn enter_varient(&mut self, var: &Variant) {
        let old = self.catch_start();

        self.cons_env.insert(var.cons);

        for arg in &var.args {
            self.check_type(arg);
        }

        self.catch_finish(old,
            CheckError::ErrorIn(var.span, "Varient"));
    }

    pub fn leave_varient(&mut self, var: &Variant) {
        self.cons_env.remove(&var.cons);
    }

    pub fn enter_pattern(&mut self, pat: &Pattern) {
        match pat {
            Pattern::Lit(lit) => {
                if let LitVal::Real(_) = lit {
                    self.throw_err(CheckError::RealNumberInPattern);
                }
            }
            Pattern::App(cons, args) => {
                if !self.cons_env.contains(&cons) {
                    self.throw_err(CheckError::ConsNotBound(*cons));
                }
                for arg in args {
                    self.enter_pattern(arg);
                }
            }
            Pattern::Var(x) => {
                self.val_env.insert(*x);
            }
            Pattern::Wild => {}
        }
    }

    pub fn leave_pattern(&mut self, pat: &Pattern) {
        match pat {
            Pattern::Lit(lit) => {}
            Pattern::App(cons, args) => {
                for arg in args {
                    self.leave_pattern(arg);
                }
            }
            Pattern::Var(x) => {
                self.val_env.remove(x);
            }
            Pattern::Wild => {}
        }
    }
    
    pub fn enter_type_decl(&mut self, decl: &DeclType) {
        let old = self.catch_start();
        
        self.type_env.insert(decl.name);
        self.check_unique(&decl.args);

        for arg in &decl.args {
            self.type_env.insert(*arg);
        }
        self.check_type(&decl.typ);
        for arg in &decl.args {
            self.type_env.remove(arg);
        }

        self.catch_finish(old,
            CheckError::ErrorIn(decl.span, "Type Decl"));
    }

    pub fn leave_type_decl(&mut self, decl: &DeclType) {
        self.type_env.remove(&decl.name);
    }

    pub fn check_type(&self, typ: &Type) {
        
        /*
        match typ {
            
            Type::Var(x) => {
                self.check_var_scope(x)?;
                Ok(())
            }
            Type::App(cons, args) => {
                if self.cons_env.contains(cons) == 0 {
                    return Err("Constructor not defined!".to_string());
                }
                for arg in args {
                    self.check_type(arg)?;
                }
                Ok(())
            }
            Type::Arr(ty1, ty2) => {
                self.check_type(ty1)?;
                self.check_type(ty2)?;
                Ok(())
            }
            Type::Lit(_) => {
                Ok(())
            }
            Type::Cons(_) => todo!(),
            Type::Temp(_) => todo!(),
            
        }
        */
    }

    pub fn check_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Lit(_) => {}
            Expr::Prim(_) => {}
            Expr::Var(expr) => {
                self.check_expr_var(expr)
            }
            Expr::Lam(expr) => {
                self.check_expr_lam(expr)
            }
            Expr::App(expr) => {
                self.check_expr_app(expr)
            }
            Expr::Let(expr) => {
                self.check_expr_let(expr)
            }
            Expr::Case(expr) => {
                todo!()
            }
            _ => unimplemented!()
        }
    }

}
pub fn check_unique(xs: &Vec<Symbol>) -> Option<Symbol> {
    let mut ys: Vec<Symbol> = Vec::new();
    for x in xs {
        for y in &ys {
            if x == y {
                return Some(*x);
            }
        }
        ys.push(*x);
    }
    None
}


#[test]
fn check_test() {
    //let string = "fn f x => (f 42 (true) 3.1415)";
    let string = "
        let
            val x = 42
            type MyInt = Int
            data Color = Red | Blue | Green
        in
            + y x
        end
    ";
    let mut par = crate::parser::Parser::new(string);
    let res = crate::parser::parse_program(&mut par);
    if let Ok(res) = res {
        let mut chk = Checker::new();
        chk.check_expr(&res);
        println!("{:#?}",chk.error);
    } else {
        par.print_err();
    }
    
}