use std::fmt;

#[derive(Clone)]
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

impl PartialEq for AST {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AST::Int(n), AST::Int(m)) => n == m,
            (AST::Name(n), AST::Name(m)) => n == m,
            (AST::NameWithType(s, n), AST::NameWithType(z, m)) => n == m && z == s,
            (AST::FunctionCall(s, n), AST::FunctionCall(z, m)) => n == m && z == s,
            (AST::Float(n), AST::Float(m)) => n == m,
            (AST::Char(n), AST::Char(m)) => n == m,
            (AST::Str(n), AST::Str(m)) => n == m,
            (AST::Boolean(n), AST::Boolean(m)) => n == m,
            (AST::Case(n, c0), AST::Case(m, c1)) => n == m && c0 == c1,
            (AST::Match(n, lc0), AST::Match(m, lc1)) => n == m && lc0 == lc1,
            (AST::Function(n, a0, t0, b0), AST::Function(m, a1, t1, b1)) => {
                n == m && a0 == a1 && t0 == t1 && b0 == b1
            }
            (AST::Assignment(s0, n), AST::Assignment(s1, m)) => n == m && s0 == s1,
            (AST::Struct(n), AST::Struct(m)) => n == m,
            (AST::Union(n), AST::Union(m)) => n == m,
            _ => false,
        }
    }
}

pub fn display_vec_ast(ast: &Vec<Box<AST>>, i: usize) -> () {
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
            display_vec_ast(stm, i + 1);
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
            display_vec_ast(&s, i + 1);
        }
        AST::Union(u) => {
            display_vec_ast(&u, i + 1);
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
