use crate::ast::AST;

const GLOBAL_NAMES: [&str; 26] = [
    "printf",
    "strcpy",
    "goto",
    "neq",
    "sizeof",
    "label",
    "getchar",
    "EOF",
    "True",
    "False",
    "nop",
    "geq",
    "leq",
    "sub",
    "Ptr.arayElem",
    "and",
    "Array",
    "Char",
    "inc",
    "seti",
    "or",
    "default",
    "eq",
    "return",
    "add_Int",
    "add",
]; //TODO

pub fn name_checker_main(ast: &Vec<Box<AST>>) -> Result<(), String> {
    let list_of_all_function_names: Vec<(String, Vec<(String, bool)>)> = vec![];
    let list_of_all_global_names = all_global_names(&ast);
    println!("list_of_all_global_names {:?}", list_of_all_global_names);
    let list_of_all_function_names = allFunctionNamesInAST(&ast, list_of_all_function_names);
    let list_of_all_structs_and_unions = all_structs_and_unions(&ast);
    println!(
        "list_of_all_function_names {:?}",
        list_of_all_function_names
    );
    error_no_main_function(&ast)?;
    error_global_name_repetition(list_of_all_global_names.clone())?;
    error_build_in_name_redefinition(&list_of_all_function_names)?;
    error_variable_not_declared(
        &ast,
        list_of_all_structs_and_unions,
        list_of_all_global_names,
    )?;
    Ok(())
}

fn all_structs_and_unions(ast: &Vec<Box<AST>>) -> Vec<(String, Vec<String>)> {
    let mut list_of_all_structs_and_unions: Vec<(String, Vec<String>)> = vec![];
    for global in ast {
        let mut list_of_fields: Vec<String> = vec![];
        match global.as_ref() {
            AST::Assignment(l, r) => match r.as_ref() {
                AST::Struct(fields) => {
                    for nwt in fields {
                        match nwt.as_ref() {
                            AST::NameWithType(n, _) => {
                                list_of_fields.push(n.to_string());
                            }
                            _ => {}
                        }
                    }
                    list_of_all_structs_and_unions.push((l.to_string(), list_of_fields));
                }
                AST::Union(fields) => {
                    for nwt in fields {
                        match nwt.as_ref() {
                            AST::NameWithType(n, _) => {
                                list_of_fields.push(n.to_string());
                            }
                            _ => {}
                        }
                    }
                    list_of_all_structs_and_unions.push((l.to_string(), list_of_fields));
                }
                _ => {}
            },
            _ => {}
        }
    }
    return list_of_all_structs_and_unions;
}

fn error_no_main_function(ast: &Vec<Box<AST>>) -> Result<(), String> {
    for global in ast {
        match global.as_ref() {
            AST::Function(name, _, _, _) => {
                if name == "main" {
                    return Ok(());
                }
            }
            _ => {}
        }
    }
    Err("No main function".to_string())
}

fn is_name_declared(
    name: &String,
    list_of_all_local_declared_variables: &Vec<String>,
) -> Result<(), String> {
    if GLOBAL_NAMES.contains(&name.as_str()) {
        return Ok(());
    }
    if list_of_all_local_declared_variables.contains(&name) {
        return Ok(());
    }
    Err(format!("Variable {} not declared", name))
}

//TODO: there is a possibility that is_name_declared could be replaced by this function:
fn is_calling_function_name_declared(
    name: &String,
    list_of_all_local_declared_variables: &Vec<String>,
    list_of_all_global_names: &Vec<String>,
) -> Result<(), String> {
    if GLOBAL_NAMES.contains(&name.as_str()) {
        return Ok(());
    }
    if list_of_all_local_declared_variables.contains(&name) {
        return Ok(());
    }
    if list_of_all_global_names.contains(&name) {
        return Ok(());
    }
    Err(format!("Variable {} not declared", name))
}

fn is_object_initialization(
    list_of_all_local_declared_variables: &mut Vec<String>,
    list_of_all_fields_in_structs_and_unions: &Vec<(String, Vec<String>)>,
    name: &String,
    name_left_side_of_assignment: &String,
) -> bool {
    let mut found = false;
    for (n, fields) in list_of_all_fields_in_structs_and_unions {
        if name == n {
            found = true;
            for field in fields {
                list_of_all_local_declared_variables
                    .push(format!("{}.{}", name_left_side_of_assignment, field));
            }
        }
    }
    found
}

fn check_local_names(
    ast: &Box<AST>,
    list_of_all_local_declared_variables: &mut Vec<String>,
    list_of_all_fields_in_structs_and_unions: &Vec<(String, Vec<String>)>,
    list_of_all_global_names: &Vec<String>,
) -> Result<(), String> {
    match ast.as_ref() {
        AST::FunctionCall(name, args) => {
            is_name_declared(name, list_of_all_local_declared_variables)?;
            check_local_names_in_stm(
                args,
                list_of_all_local_declared_variables,
                list_of_all_fields_in_structs_and_unions,
                list_of_all_global_names,
            )?;
        }
        AST::Name(name) => {
            is_name_declared(name, list_of_all_local_declared_variables)?;
        }
        _ => {}
    }
    Ok(())
}

fn check_local_names_in_assignment(
    ast: &Box<AST>,
    list_of_all_local_declared_variables: &mut Vec<String>,
    list_of_all_fields_in_structs_and_unions: &Vec<(String, Vec<String>)>,
    list_of_all_global_names: &Vec<String>,
    name_left_side_of_assignment: &String,
) -> Result<(), String> {
    match ast.as_ref() {
        AST::FunctionCall(name, args) => {
            if !is_object_initialization(
                list_of_all_local_declared_variables,
                list_of_all_fields_in_structs_and_unions,
                name,
                name_left_side_of_assignment,
            ) {
                is_name_declared(name, list_of_all_local_declared_variables)?;
            }
            check_local_names_in_stm(
                args,
                list_of_all_local_declared_variables,
                list_of_all_fields_in_structs_and_unions,
                list_of_all_global_names,
            )?;
        }
        AST::Name(name) => {
            is_name_declared(name, list_of_all_local_declared_variables)?;
        }
        _ => {}
    }
    Ok(())
}

fn check_local_names_in_stm(
    ast: &Vec<Box<AST>>,
    list_of_all_local_declared_variables: &mut Vec<String>,
    list_of_all_fields_in_structs_and_unions: &Vec<(String, Vec<String>)>,
    list_of_all_global_names: &Vec<String>,
) -> Result<(), String> {
    for local in ast {
        match local.as_ref() {
            AST::Assignment(l, r) => {
                check_local_names_in_assignment(
                    r,
                    list_of_all_local_declared_variables,
                    list_of_all_fields_in_structs_and_unions,
                    list_of_all_global_names,
                    l,
                )?;
                if GLOBAL_NAMES.contains(&l.as_str()) {
                    return Err(format!("Redeclaration of global variable {}", l));
                }
                list_of_all_local_declared_variables.push(l.to_string());
            }
            AST::Match(expr, cases) => {
                check_local_names_in_stm(
                    expr,
                    list_of_all_local_declared_variables,
                    list_of_all_fields_in_structs_and_unions,
                    list_of_all_global_names,
                )?;
                check_local_names_in_stm(
                    cases,
                    list_of_all_local_declared_variables,
                    list_of_all_fields_in_structs_and_unions,
                    list_of_all_global_names,
                )?;
            }
            AST::Case(expr, stm) => {
                check_local_names(
                    expr,
                    list_of_all_local_declared_variables,
                    list_of_all_fields_in_structs_and_unions,
                    list_of_all_global_names,
                )?;
                check_local_names_in_stm(
                    stm,
                    list_of_all_local_declared_variables,
                    list_of_all_fields_in_structs_and_unions,
                    list_of_all_global_names,
                )?;
            }
            AST::FunctionCall(name, args) => {
                is_calling_function_name_declared(
                    name,
                    list_of_all_local_declared_variables,
                    list_of_all_global_names,
                )?;
                check_local_names_in_stm(
                    args,
                    list_of_all_local_declared_variables,
                    list_of_all_fields_in_structs_and_unions,
                    list_of_all_global_names,
                )?;
            }
            AST::Name(name) => {
                is_name_declared(name, list_of_all_local_declared_variables)?;
            }
            AST::NameWithType(_, _) => {}
            _ => {}
        }
    }
    Ok(())
}

fn list_of_all_arguments(args: &Vec<Box<AST>>) -> Vec<String> {
    let mut list_of_all_arguments = Vec::new();
    for arg in args {
        match arg.as_ref() {
            AST::NameWithType(n, _) => list_of_all_arguments.push(n.to_string()),
            _ => {}
        }
    }
    list_of_all_arguments
}

fn error_variable_not_declared(
    ast: &Vec<Box<AST>>,
    list_of_all_fields_in_structs_and_unions: Vec<(String, Vec<String>)>,
    list_of_all_global_names: Vec<String>,
) -> Result<(), String> {
    for global in ast {
        match global.as_ref() {
            AST::Function(_, args, _, stm) => {
                let mut list_of_all_local_declared_variables: Vec<String> =
                    list_of_all_labels_in_function(stm);
                list_of_all_local_declared_variables.append(&mut list_of_all_arguments(args));
                check_local_names_in_stm(
                    stm,
                    &mut list_of_all_local_declared_variables,
                    &list_of_all_fields_in_structs_and_unions,
                    &list_of_all_global_names,
                )?
            }
            _ => {}
        }
    }
    Ok(())
}

fn list_of_all_labels_in_function(ast: &Vec<Box<AST>>) -> Vec<String> {
    let mut list_of_all_labels_in_function: Vec<String> = vec![];
    for stm in ast {
        match stm.as_ref() {
            AST::Assignment(l, r) => match r.as_ref() {
                AST::FunctionCall(name, _) => {
                    if name == "label" {
                        list_of_all_labels_in_function.push(l.to_string());
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
    list_of_all_labels_in_function
}

fn all_global_names(ast: &Vec<Box<AST>>) -> Vec<String> {
    let mut list_of_all_global_names: Vec<String> = vec![];
    for global in ast {
        match global.as_ref() {
            AST::Assignment(n, l) => {
                list_of_all_global_names.push(n.to_string());
            }
            AST::Function(name, _, _, _) => {
                list_of_all_global_names.push(name.to_string());
            }
            _ => {}
        }
    }
    return list_of_all_global_names;
}

fn allFunctionNamesInAST(
    ast: &Vec<Box<AST>>,
    mut loafn: Vec<(String, Vec<(String, bool)>)>,
) -> Vec<(String, Vec<(String, bool)>)> {
    for leaf in ast {
        match leaf.as_ref() {
            AST::Function(name, _, t, stm) => {
                let mut listOfAllNames: Vec<(String, bool)> = vec![];
                loafn.push((name.to_string(), allNamesInAST(&stm, listOfAllNames)));
            }
            _ => {}
        }
    }
    loafn
}

fn allNamesInAST(ast: &Vec<Box<AST>>, mut loan: Vec<(String, bool)>) -> Vec<(String, bool)> {
    for leaf in ast {
        loan = allNamesInASTBox(leaf, loan);
    }
    loan
}

fn allNamesInASTBox(ast: &Box<AST>, mut loan: Vec<(String, bool)>) -> Vec<(String, bool)> {
    match ast.as_ref() {
        AST::Name(s) => {
            loan.push((s.to_string(), false));
        }
        AST::NameWithType(n, t) => {
            loan.push((n.to_string(), false));
        }
        AST::FunctionCall(s, v) => {
            loan.push((s.to_string(), false));
            loan = allNamesInAST(&v, loan);
        }
        AST::Case(arg, stm) => {
            loan = allNamesInASTBox(&arg, loan);
            loan = allNamesInAST(&stm, loan);
        }
        AST::Match(arg, case) => {
            loan = allNamesInAST(&arg, loan);
            loan = allNamesInAST(&case, loan);
        }
        AST::Function(name, _, _, stm) => {
            loan.push((name.to_string(), false));
            loan = allNamesInAST(&stm, loan);
        }
        AST::Assignment(name, fc) => {
            loan.push((name.to_string(), false));
            loan = allNamesInASTBox(&fc, loan);
        }
        AST::Struct(s) => {
            loan = allNamesInAST(&s, loan);
        }
        AST::Union(u) => {
            loan = allNamesInAST(&u, loan);
        }
        AST::Int(i) => {}
        AST::Float(f) => {}
        AST::Char(c) => {}
        AST::Str(s) => {}
        AST::Boolean(b) => {}
    }
    loan
}

fn error_build_in_name_redefinition(
    list_of_all_function_names: &Vec<(String, Vec<(String, bool)>)>,
) -> Result<(), String> {
    for (name, _) in list_of_all_function_names {
        if GLOBAL_NAMES.contains(&name.as_str()) {
            return Err(format!("Redefinition of build-in {}", name));
        }
    }
    Ok(())
}

fn error_no_goto_label_in_function() -> () {
    //TODO
}

fn error_repeated_name_in_struct_or_union() -> () {
    //TODO
}

fn error_global_name_repetition(mut loagn: Vec<String>) -> Result<(), String> {
    if loagn.len() == 1 {
        return Ok(());
    }
    loagn.sort();
    for i in 0..loagn.len() - 2 {
        if loagn[i] == loagn[i + 1] {
            return Err(format!("Repetition of global name {}", loagn[i]));
        }
    }
    Ok(())
}

fn warning_variable_declared_but_not_used() -> () {
    //TODO
}

fn warningUnusedLabel() -> () {}
