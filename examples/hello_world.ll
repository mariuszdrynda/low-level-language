target triple = "x86_64-pc-linux-gnu"
@.str = private unnamed_addr constant [14 x i8] c"Hello World!\0A\00"
define dso_local i32 @main() {
    call i32 (ptr, ...) @printf(ptr noundef @.str)
    call i32 @getchar()
    ret i32 0
}
declare i32 @printf(ptr noundef, ...)
declare i32 @getchar()