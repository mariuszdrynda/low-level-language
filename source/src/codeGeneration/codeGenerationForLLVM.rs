use crate::ast::AST;

pub fn code_generation_for_llvm(ast: Vec<Box<AST>>) {
    generate_ssa_form(ast);
    let output = String::new();
    let all_c_lib_functions = all_c_lib_functions();
    //create .ll file and save output to that file
}

fn generate_ssa_form(ast: Vec<Box<AST>>) {
    todo!()
}

fn all_c_lib_functions() {
    todo!()
}
