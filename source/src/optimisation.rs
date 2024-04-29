use crate::ast::{Ty, AST};

pub fn optimisation_main(ast: Vec<Box<AST>>) -> Vec<Box<AST>> {
    let ast = constant_propagation(ast);
    let ast = all_string_literals_to_global(ast);
    let ast = call_functions_with_all_arguments_known(ast);
    vec![]
}

fn constant_propagation(ast: Vec<Box<AST>>) -> Vec<Box<AST>> {
    let ast: Vec<Box<AST>> = ast
        .iter()
        .map(|global| match global.as_ref() {
            AST::Function(n, a, t, stm) => Box::new(AST::Function(
                n.to_string(),
                a,
                t.clone(),
                constant_propagation_in_function(*stm),
            )),
            _ => *global,
        })
        .collect();
    ast
}

fn constant_propagation_in_function(ast: Vec<Box<AST>>) -> Vec<Box<AST>> {
    todo!();
    vec![]
}

fn all_string_literals_to_global(ast: Vec<Box<AST>>) -> Vec<Box<AST>> {
    // TODO throw all string literals to global scope
    // replace all string literals in functions to ptr to global variables
    todo!();
    vec![]
}

fn call_functions_with_all_arguments_known(ast: Vec<Box<AST>>) -> Vec<Box<AST>> {
    // TODO if all argument of the functions are known in compiletime we can execute that function
    // if it not produces any side effects
    todo!();
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    fn constant_propagation_test() {
        /*
           x = 5;
           y = add(3, x);
        */
        let ast = vec![
            Box::new(AST::Assignment(String::from("x"), Box::new(AST::Int(5)))),
            Box::new(AST::Assignment(
                String::from("y"),
                Box::new(AST::FunctionCall(
                    String::from("add"),
                    vec![
                        Box::new(AST::Int(3)),
                        Box::new(AST::Name(String::from("x"))),
                    ],
                )),
            )),
        ];
        let result = vec![
            Box::new(AST::Assignment(String::from("x"), Box::new(AST::Int(5)))),
            Box::new(AST::Assignment(
                String::from("y"),
                Box::new(AST::FunctionCall(
                    String::from("add"),
                    vec![Box::new(AST::Int(3)), Box::new(AST::Int(5))],
                )),
            )),
        ];
        let ast = constant_propagation_in_function(ast);
        assert_eq!(ast, result);
    }
}
