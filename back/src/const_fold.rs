use crate::cps_trans::*;

use crate::cps_trans::Atom::*;
use crate::cps_trans::InstrBody::*;

impl Optimizer {

    pub fn const_fold_instr(&mut self, ins: InstrId) -> InstrId {
        match self.get_body(ins) {
            IAdd(r, Int(x), Int(y)) => {
                assert!(self.get_next(ins).is_some());
                self.delete_instr(ins);
                let next = self.get_next(ins).unwrap();
                self.instr_subst(next, r, Int(x + y));
                next
            }
            ISub(r, Int(x), Int(y)) => {
                assert!(self.get_next(ins).is_some());
                self.delete_instr(ins);
                let next = self.get_next(ins).unwrap();
                self.instr_subst(ins, r, Int(x - y));
                next
            }
            Ifte(Bool(p), trbr, flbr) => {
                assert!(self.get_next(ins).is_none());
                if p {
                    self.update_instr(ins, trbr);
                    trbr
                } else {
                    self.update_instr(ins, flbr);
                    flbr
                }
            },
            Switch(Int(n), brs) => {
                assert!(self.get_next(ins).is_none());
                assert!(n >= 0);
                self.update_instr(ins, brs[n as usize]);
                brs[n as usize]
            }
            _ => {
                ins /* Nothing happens */
            }
        }
    }

    pub fn instr_subst(&mut self, ins: InstrId, name: Symbol, value: Atom) {
        let mut with = ins;
        loop {
            match &mut self.get_body(with) {
                Goto(k, args) => {
                    k.subst(name, value);
                    for arg in args {
                        arg.subst(name, value);
                    }
                },
                IAdd(r,x,y) => {
                    x.subst(name, value);
                    y.subst(name, value);
                    if *r == name { break; }
                }
                ISub(r, x, y) => {
                    x.subst(name, value);
                    y.subst(name, value);
                    if *r == name { break; }
                }
                Ifte(p, trbr, flbr) => {
                    p.subst(name, value);
                    self.instr_subst(*trbr, name, value);
                    self.instr_subst(*flbr, name, value);
                }
                Switch(n, brs) => {
                    n.subst(name, value);
                    for br in brs {
                        self.instr_subst(*br, name, value);
                    }
                }
                Halt(x) => {
                    x.subst(name, value);
                }
            }

            if let Some(next) = self.get_next(with) {
                with = next;
            } else {
                break;
            }
            
        }

    }
}