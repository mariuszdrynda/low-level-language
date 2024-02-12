enum Ins {
    ADD(InsTy),
    ARRAY(InsTy, usize), // returns pointer to an array TODO multidimensional arrays
    BOOL_AND,            // (Bool, Bool) -> Bool
    BOOL_NOT,            // (Bool) -> Bool
    BOOL_OR,             // (Bool, Bool) -> Bool
    CALL(usize),
    FLOW_RET(usize),
    HALT,
    SUB(InsTy),
    SYSCALL(Syscall),
    UNINITIALIZE(InsTy),
    VAR(VarTy),
}

enum VarTy {
    F32(f32),
    F64(f64),
    I32(i32),
    I64(i64),
}

enum Syscall {
    _exit,
    exec,
    exit,
    fork,
    mmap,
    seteuid,
    setuid,
    munmap,
    wait,
}

enum InsTy {
    F32,
    F64,
    I32,
    I64,
}

fn testProgram() -> Vec<Ins> {
    let mut program: Vec<Ins> = Vec::new();
    program.push(Ins::VAR(VarTy::I64(12)));
    program.push(Ins::VAR(VarTy::I64(7)));
    program.push(Ins::SUB(InsTy::I64));
    program.push(Ins::HALT);
    program
}

fn execute(program: Vec<Ins>) -> () {
    let mut stack: Vec<i64> = Vec::new();
    let mut ip: usize = 0;
    loop {
        println!("IP: {:?}, STACK: {:?}", ip, stack);
        match &program[ip] {
            Ins::VAR(i) => match i {
                VarTy::I64(j) => {
                    stack.push(j.clone());
                    ip = ip + 1;
                }
                _ => {}
            },
            Ins::ADD(_) => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(x + y);
                ip = ip + 1;
            }
            Ins::SUB(_) => {
                let y = stack.pop().unwrap();
                let x = stack.pop().unwrap();
                stack.push(x - y);
                ip = ip + 1;
            }
            Ins::HALT => break,
            _ => break, //TODO
        }
    }
}

pub fn executorMain() -> () {
    let program = testProgram();
    execute(program);
}
