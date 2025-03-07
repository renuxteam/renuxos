; ModuleID = 'multiboot_header.c'
source_filename = "multiboot_header.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

@multiboot_header = dso_local global [4 x i32] [i32 -397258538, i32 0, i32 0, i32 0], section ".multiboot2", align 16
@llvm.compiler.used = appending global [1 x ptr] [ptr @multiboot_header], section "llvm.metadata"

; Function Attrs: noinline nounwind optnone sspstrong uwtable
define dso_local void @_start() #0 {
  br label %1

1:                                                ; preds = %0, %1
  br label %1
}

attributes #0 = { noinline nounwind optnone sspstrong uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }

!llvm.module.flags = !{!0, !1, !2, !3, !4}
!llvm.ident = !{!5}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 8, !"PIC Level", i32 2}
!2 = !{i32 7, !"PIE Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 2}
!4 = !{i32 7, !"frame-pointer", i32 2}
!5 = !{!"clang version 19.1.7"}
