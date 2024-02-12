use std::env;
use std::fs;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub grammar);
pub mod ast;
pub mod executor;
pub mod nameChecker;
pub mod typeChecker;
use crate::nameChecker::name_checker_main;
use crate::typeChecker::type_checker_main;
fn compilation_phases(input: String) -> () {
    println!("PHASE 1: syntax parse");
    //let mut errors = Vec::new();
    let ast = grammar::TopParser::new().parse(&input);
    match &ast {
        Ok(a) => {
            // println!("Phase 1 ended succesfully. AST:\n {:?}", ast1);
            ast::display_Vec_AST(&a, 0);
        }
        Err(err) => {
            println!("Phase 1 (syntax parse) ended with an error:\n{err}.\nAborting compilation.");
            return;
        }
    }
    println!("PHASE 2: name checker");
    let ast = ast.unwrap();
    match name_checker_main(&ast) {
        Err(s) => {
            println!("Phase 2 (name checker) ended with an error:\n{s}.\nAborting compilation.");
            return;
        }
        _ => {}
    }
    println!("PHASE 3: dead code elimination - omitted");
    println!("PHASE 4: type checker");
    match type_checker_main(&ast) {
        Err(s) => {
            println!("Phase 4 (type checker) ended with an error:\n{s}.\nAborting compilation.");
            return;
        }
        _ => {}
    }
    // println!("PHASE 5: error checker");
    // println!("PHASE 6: possible undefined behaviour");
    // println!("PHASE 7: optimisation");
    // println!("PHASE 8: code generation");
    // println!("PHASE 9: code execution");
    // executorMain();
    ()
}
fn main() {
    println!("LLL COMPILER VER 0.0.1");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let file_path = &args[1];
        println!("Start compilation of file {}", file_path);
        let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
        compilation_phases(input);
    } else {
        println!("No input file specified");
    }
}
#[test]
fn lall_grammar_test() {
    assert!(grammar::TopParser::new()
        .parse("fn main()->Unit{declare.i32(celsius);}")
        .is_ok());
    assert!(grammar::TopParser::new()
        .parse("fn main()->Unit{return(0);}")
        .is_ok());
    assert!(grammar::TopParser::new()
        .parse("include(printf) declare_set_global(a, 10)")
        .is_ok());
    assert!(grammar::TopParser::new()
        .parse(
            "fn main()->Unit{set_f32(celsius, mul_f32(div_f32(5.0, 9.0), sub_f32(fahr, 32.0)));}"
        )
        .is_ok());
    assert!(grammar::TopParser::new()
        .parse("fn main()->Unit{declare_set_f32(fahr, cast_i32_f32(lower)); loop l1{nop(());}}")
        .is_ok());
    // assert!(grammar::TopParser::new().parse("fn main()->(){declare_set_i32(lower, 0i32); declare_set_i32(upper, 300); declare_set_i32(step, 20);}").is_ok());
    // assert!(grammar::TopParser::new().parse("declare_set_global(a, '\n')").is_ok());
    // assert!(grammar::TopParser::new().parse("fn main()->(){case(leq_f32(fahr, upper)){false -> {break(l1);} true -> {nop(());}}}").is_ok());
    // assert!(grammar::TopParser::new().parse("fn main()->(){printf(\"%3.0f %6.1f\n\", fahr, celsius);}").is_ok());
    //TODO: all files with .lll extension from /examples folder
}
