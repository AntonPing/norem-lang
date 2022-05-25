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
                self.subst(next, r, Int(x + y));
                next
            }
            ISub(r, Int(x), Int(y)) => {
                assert!(self.get_next(ins).is_some());
                self.delete_instr(ins);
                let next = self.get_next(ins).unwrap();
                self.subst(ins, r, Int(x - y));
                next
            }
            Ifte(Bool(p), trbr, flbr) => {
                assert!(self.get_next(ins).is_none());
                if p {
                    self.subst_instr(ins, trbr);
                    trbr
                } else {
                    self.subst_instr(ins, flbr);
                    flbr
                }
            },
            Switch(Int(n), brs) => {
                assert!(self.get_next(ins).is_none());
                assert!(n >= 0);
                self.subst_instr(ins, brs[n as usize]);
                brs[n as usize]
            }
            _ => {
                ins /* Nothing happens */
            }
        }
    }

    pub fn subst(&mut self, ins: InstrId, name: Symbol, value: Atom) {
        let mut with = ins;
        loop {
            match &mut self.get_body(with) {
                Goto(k, args) => {

                },
                IAdd(r,x,y) => {
                    x.subst(name, value);
                    y.subst(name, value);
                    if *r == name { break; }
                }
                
                ISub(_, _, _) => todo!(),
                Ifte(_, _, _) => todo!(),
                Switch(_, _) => todo!(),
                Halt(_) => todo!(),

            }

            if let Some(next) = self.get_next(with) {
                with = next;
            } else {
                break;
            }
            
        }

    }
}