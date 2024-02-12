use std::fmt;

pub enum AST {
    Int(i64),
    Name(String),
    NameWithType(String, Ty),
    FunctionCall(String, Vec<Box<AST>>),
    Float(f64),
    Char(char),
    Str(String),
    Boolean(bool),
    Case(Box<AST>, Vec<Box<AST>>),
    Match(Vec<Box<AST>>, Vec<Box<AST>>),
    Function(String, Vec<Box<AST>>, Ty, Vec<Box<AST>>),
    Assignment(String, Box<AST>),
    Struct(Vec<Box<AST>>),
    Union(Vec<Box<AST>>),
}

#[derive(Clone, Eq)]
pub enum Ty {
    Name(String),
    Func(Vec<Ty>, Box<Ty>),
}

impl PartialEq for Ty {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Ty::Name(n), Ty::Name(m)) => n == m,
            (Ty::Func(v1, rt1), Ty::Func(v2, rt2)) => {
                if v1.len() != v2.len() {
                    return false;
                }
                // v1.iter()
                //     .zip(v2.iter())
                //     .map(|(&a, &b)| a == b)
                //     .fold(true, |acc, a| if a == acc { true } else { false });
                for i in 0..v1.len() {
                    if v1[i] != v2[i] {
                        return false;
                    }
                }
                if rt1 != rt2 {
                    return false;
                }
                return true;
            }
            _ => false,
        }
    }
}

pub fn display_Vec_AST(ast: &Vec<Box<AST>>, i: usize) -> () {
    // for _ in 0..i {
    //     print!("\t");
    // }
    // print!("[\n");
    for a in 0..ast.len() {
        display_AST(&ast[a], i);
    }
    // for _ in 0..i {
    //     print!("\t");
    // }
    // print!("]\n");
}

fn display_AST(ast: &AST, i: usize) -> () {
    for _ in 0..i {
        print!("\t");
    }
    match ast {
        AST::Int(i) => {
            print!("{:?}\n", i);
        }
        AST::Name(s) => {
            print!("{:?}\n", s);
        }
        AST::NameWithType(n, t) => {}
        AST::FunctionCall(s, v) => {
            print!("(CALL {:?})\n", s);
        }
        AST::Float(f) => {
            print! {"{:?}\n", f};
        }
        AST::Char(c) => {
            print! {"{:?}\n", c};
        }
        AST::Str(s) => {
            print! {"{:?}\n", s};
        }
        AST::Boolean(b) => {
            print! {"{:?}\n", b};
        }
        AST::Case(arg, stm) => {}
        AST::Match(arg, case) => {
            print!("(MATCH)\n");
        }
        AST::Function(name, a, t, stm) => {
            print!("(FUNCTION {:?}\n", name);
            display_Vec_AST(stm, i + 1);
            for _ in 0..i {
                print!("\t");
            }
            print!(")\n");
        }
        AST::Assignment(name, fc) => {
            print!("(ASSIGNMENT {:?}\n", name);
            for _ in 0..i {
                print!("\t");
            }
            print!(")\n");
        }
        AST::Struct(s) => {
            display_Vec_AST(&s, i + 1);
        }
        AST::Union(u) => {
            display_Vec_AST(&u, i + 1);
        }
    }
}

impl fmt::Debug for Ty {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::Ty::*;
        match &*self {
            Name(s) => write!(fmt, "{:?}", s),
            Func(v, t) => write!(fmt, "({:?} {:?})", v, t),
        }
    }
}

impl fmt::Debug for AST {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::AST::*;
        match &*self {
            Int(i) => write!(fmt, "{:?}", i),
            Name(s) => write!(fmt, "{:?}", s),
            NameWithType(n, t) => write!(fmt, "(ID {:?} {:?})", n, t),
            FunctionCall(s, v) => write!(fmt, "(CALL {:?} {:?})", s, v),
            Float(f) => write!(fmt, "{:?}", f),
            Char(c) => write!(fmt, "{:?}", c),
            Str(s) => write!(fmt, "{:?}", s),
            Boolean(b) => write!(fmt, "{:?}", b),
            Case(arg, stm) => write!(fmt, ""),
            Match(arg, case) => write!(fmt, ""),
            Function(name, a, t, stm) => write!(fmt, "(FUNCTION {:?} {:?} {:?})", name, t, stm),
            Assignment(name, fc) => write!(fmt, "(ASSIGNMENT {:?} {:?})", name, fc),
            Struct(s) => write!(fmt, "{:?}", s),
            Union(u) => write!(fmt, "{:?}", u),
        }
    }
}
