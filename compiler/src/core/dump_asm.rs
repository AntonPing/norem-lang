use std::collections::{HashSet, HashMap};
use std::fs::{File, self};
use std::io::{Write, ErrorKind};

use super::*;
use crate::symbol::*;
use super::reg_alloc::{AllocScan, RegAlloc};
use super::visitor::*;

/*
    dump bytecode to asm
*/

pub fn reg_x86_64(n: usize) -> String {
    match n {
        0 => "%r8".to_string(),
        1 => "%r9".to_string(),
        2 => "%r10".to_string(),
        3 => "%r11".to_string(),
        4 => "%r12".to_string(),
        5 => "%r13".to_string(),
        6 => "%r14".to_string(),
        7 => "%r15".to_string(),
        other => {
            let n = other - 8;
            format!("0(%rbx,{n},8)")
        },
    }
}

pub fn atom_x86_64(atom: Atom) -> String {
    match atom {
        Atom::Var(Symbol::Reg(n)) => 
            reg_x86_64(n),
        Atom::Var(_) =>
            panic!("free variable!"),
        Atom::Glob(sym) =>
            format!("{sym}"),
        Atom::Prim(_) => 
            panic!("no primitive!"),
        Atom::Int(x) => 
            format!("${x}"),
        Atom::Real(x) => // todo
            format!("${x}"),
        Atom::Bool(x) =>
            if x { "$1".to_string() }
            else { "$0".to_string() }
        Atom::Char(x) => // todo
            format!("${x}"),
    }
}

pub fn code_x86_64(code: ByteCode) -> String {
    let indent = "    ";
    match code {
        ByteCode::Move(x, y) => {
            let x = atom_x86_64(x);
            let y = reg_x86_64(y);
            format!("\
                {indent}movq {x}, {y}\n\
            ")
        }
        ByteCode::Jump(x) => {

            

            let x = atom_x86_64(x);



            format!("\
                {indent}jmp {x}\n\
            ")
        }
        ByteCode::Halt(x) => {
            let x = atom_x86_64(x);
            format!("\
                {indent}movq {x}, %rax\n\
                {indent}ret\n
            ")
        }
        ByteCode::IAdd(x, y, z) => {
            let x = atom_x86_64(x);
            let y = atom_x86_64(y);
            let z = reg_x86_64(z);
            format!("\
                {indent}movq {x}, {z}\n\
                {indent}addq {y}, {z}\n\
            ")
        }
        ByteCode::ISub(x, y, z) => {
            let x = atom_x86_64(x);
            let y = atom_x86_64(y);
            let z = reg_x86_64(z);
            format!("\
                {indent}movq {x}, {z}\n\
                {indent}subq {y}, {z}\n\
            ")
        }
        ByteCode::IMul(x, y, z) => {
            let x = atom_x86_64(x);
            let y = atom_x86_64(y);
            let z = reg_x86_64(z);
            format!("\
                {indent}movq {x}, {z}\n\
                {indent}mulq {y}, {z}\n\
            ")
        }
        ByteCode::IDiv(_, _, _) => todo!(),
        ByteCode::INeg(_, _) => todo!(),
        ByteCode::BNot(_, _) => todo!(),
    }
}

pub fn dump_x86_64(
    map: HashMap<Symbol,ByteCodeBlock>,
) -> std::io::Result<()> {
    let mut file = File::options()
        .append(true)
        .create(true)
        .open("dump_x86_64.s")?;

    // x86_64 header
    file.write(b".text\n")?;

    for (_,block) in map.into_iter() {
        let func = block.func;
        let args = block.args;
        file.write(format!(
            "{func}: # {args}\n").as_bytes())?;
        
        for code in block.body {
            file.write(code_x86_64(code).as_bytes())?;
        }
        file.write(b"\n")?;
    }

    //fs::remove_file("dump_x86_64.asm")?;

    Ok(())
}


#[test]
pub fn dump_x86_64_test() -> std::io::Result<()> {
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
        let blocks = super::codegen::CodeGen::run(expr);
        dump_x86_64(blocks)
    } else {
        par.print_err();
        Err(std::io::Error::new(ErrorKind::Other, "bad"))
    }


    
}