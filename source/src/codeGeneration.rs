enum AvailablePlatforms {
    elf_linux_x64,
    llvm, // https://crates.io/crates/llvm-sys https://llvm.org/docs/GettingStarted.html https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/LangImpl01.html
    executor, // build-in IR executor
}

fn findTargetPlatform(ast: Box<AST>) -> AvailablePlatforms {
    // TODO
    return AvailablePlatforms::executor;
}

fn mainCodeGeneration(ast: Box<AST>) -> () {
    match findTargetPlatform(ast) {
        AvailablePlatforms::elf_linux_x64 => {}
        AvailablePlatforms::llvm => {}
        AvailablePlatforms::executor => {}
    }
}
