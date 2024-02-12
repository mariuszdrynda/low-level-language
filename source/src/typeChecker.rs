use crate::ast::{Ty, AST};
use std::collections::HashMap;

//TODO types_of_build_ins

pub fn type_checker_main(ast: &Vec<Box<AST>>) -> Result<(), String> {
    let all_functions_types: HashMap<String, Ty> = all_functions_types(ast);
    println!("all_functions_types\n{:?}", all_functions_types);
    let all_structs_and_fields_types: HashMap<String, HashMap<String, Ty>> =
        all_structs_and_fields_types(ast);
    println!(
        "all_structs_and_fields_types\n{:?}",
        all_structs_and_fields_types
    );
    let all_unions_and_fields_types: HashMap<String, HashMap<String, Ty>> =
        all_unions_and_fields_types(ast);
    println!(
        "all_unions_and_fields_types\n{:?}",
        all_unions_and_fields_types
    );
    check_all_types(ast)?;
    Ok(())
}

fn all_arguments_types(args: &Vec<Box<AST>>) -> Vec<Ty> {
    let mut all_arguments_types = Vec::new();
    for arg in args {
        match arg.as_ref() {
            AST::NameWithType(n, t) => all_arguments_types.push(t.clone()),
            _ => {}
        }
    }
    all_arguments_types
}

fn all_functions_types(ast: &Vec<Box<AST>>) -> HashMap<String, Ty> {
    let mut all_functions_types: HashMap<String, Ty> = HashMap::new();
    for global in ast {
        match global.as_ref() {
            AST::Function(name, args, t, _) => {
                all_functions_types.insert(
                    name.to_string(),
                    Ty::Func(all_arguments_types(args), Box::new(t.clone())),
                );
            }
            _ => {}
        }
    }
    all_functions_types
}

fn all_structs_and_fields_types(ast: &Vec<Box<AST>>) -> HashMap<String, HashMap<String, Ty>> {
    let mut all_structs_and_fields_types: HashMap<String, HashMap<String, Ty>> = HashMap::new();
    for global in ast {
        match global.as_ref() {
            AST::Assignment(left, right) => match right.as_ref() {
                AST::Struct(fields) => {
                    let mut types_of_fields: HashMap<String, Ty> = HashMap::new();
                    for field in fields {
                        match field.as_ref() {
                            AST::NameWithType(name, ty) => {
                                types_of_fields.insert(name.to_string(), ty.clone());
                            }
                            _ => {}
                        }
                    }
                    all_structs_and_fields_types.insert(left.to_string(), types_of_fields);
                }
                _ => {}
            },
            _ => {}
        }
    }
    all_structs_and_fields_types
}

fn all_unions_and_fields_types(ast: &Vec<Box<AST>>) -> HashMap<String, HashMap<String, Ty>> {
    let mut all_unions_and_fields_types: HashMap<String, HashMap<String, Ty>> = HashMap::new();
    for global in ast {
        match global.as_ref() {
            AST::Assignment(left, right) => match right.as_ref() {
                AST::Union(fields) => {
                    let mut types_of_fields: HashMap<String, Ty> = HashMap::new();
                    for field in fields {
                        match field.as_ref() {
                            AST::NameWithType(name, ty) => {
                                types_of_fields.insert(name.to_string(), ty.clone());
                            }
                            _ => {}
                        }
                    }
                    all_unions_and_fields_types.insert(left.to_string(), types_of_fields);
                }
                _ => {}
            },
            _ => {}
        }
    }
    all_unions_and_fields_types
}

fn check_type_of_variable(name: &String) -> Ty {
    Ty::Name("".to_string()) //TODO
}

fn check_type_of_expression(expr: &Box<AST>) -> Ty {
    Ty::Name("".to_string()) //TODO
}

fn check_all_variables_in_function(ast: &Vec<Box<AST>>) -> Result<(), String> {
    Ok(()) //TODO
}

fn check_all_types(ast: &Vec<Box<AST>>) -> Result<(), String> {
    for global in ast {
        println!("CHECKING {:?}", global);
        match global.as_ref() {
            AST::Function(_, _, _, stms) => {
                check_all_variables_in_function(stms);
            }
            AST::Assignment(l, r) => {
                if check_type_of_variable(l) != check_type_of_expression(r) {
                    return Err("Assignment type mismatch".to_string());
                }
            }
            AST::Match(condition, cases) => {
                //TODO condition has to be equal every case in cases
            }
            _ => {}
        }
    }
    Ok(()) //TODO
}

fn error_case_type_mismatch() -> () {}

fn error_recursive_field_in_a_struct() -> () {}

fn error_function_call_type_mismatch() -> () {}
