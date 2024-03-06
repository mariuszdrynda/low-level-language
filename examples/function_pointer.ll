@.str = private unnamed_addr constant [3 x i8] c"%d\00", align 1

define dso_local i32 @add_5(i32 noundef %0) local_unnamed_addr{
  %2 = add nsw i32 %0, 5
  ret i32 %2
}

define dso_local i32 @main() {
  %1 = alloca ptr, align 8
  store ptr @add_5, ptr %1, align 8
  %2 = load ptr, ptr %1, align 8
  %3 = call i32 %2(i32 noundef 3)
  call i32 (ptr, ...) @printf(ptr noundef @.str, i32 noundef %3)
  ret i32 0
}

declare noundef i32 @printf(ptr nocapture noundef readonly, ...) local_unnamed_addr #2