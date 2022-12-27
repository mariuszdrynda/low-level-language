use std::fmt::{Debug, Error, Formatter};

pub enum AST {
    Int(i64),
    Name(String),
    FunctionCall(String, Vec<Box<AST>>),
    Float(f64),
    Char(char),
    Str(String),
    Boolean(bool),
    Void,
    Case(Box<AST>, Vec<Box<AST>>),
    Match(Vec<Box<AST>>, Vec<Box<AST>>),
    Loop(String, Vec<Box<AST>>),
    Function(String, String, Vec<Box<AST>>),
}

impl Debug for AST { //TODO
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::AST::*;
        match &*self {
            Int(i) => write!(fmt, "{:?}", i),
            Name(s) => write!(fmt, "{:?}", s),
            FunctionCall(s, v) => write!(fmt, ""),
            Float(f) => write!(fmt, ""),
            Char(c) => write!(fmt, ""),
            Str(s) => write!(fmt, ""),
            Boolean(b) => write!(fmt, ""),
            Void => write!(fmt, "(VOID)"),
            Case(arg, stm) => write!(fmt, ""),
            Match(arg, case) => write!(fmt, ""),
            Loop(s, v) => write!(fmt, ""),
            Function(name, t, stm) => write!(fmt, ""),
        }
    }
}