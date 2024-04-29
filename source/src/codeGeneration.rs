use crate::ast::AST;
mod codeGenerationForLLVM;
use crate::codeGeneration::codeGenerationForLLVM::code_generation_for_llvm;

enum AvailablePlatforms {
    elf_linux_x64,
    llvm, // https://crates.io/crates/llvm-sys https://llvm.org/docs/GettingStarted.html https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl01.html
    executor, // build-in IR executor
}

fn findTargetPlatform(ast: &Vec<Box<AST>>) -> AvailablePlatforms {
    todo!();
    return AvailablePlatforms::llvm;
}

pub fn main_code_generation(ast: Vec<Box<AST>>) -> Vec<Box<AST>> {
    match findTargetPlatform(&ast) {
        AvailablePlatforms::elf_linux_x64 => {}
        AvailablePlatforms::llvm => code_generation_for_llvm(ast),
        AvailablePlatforms::executor => {}
    }
    todo!();
    vec![]
}
