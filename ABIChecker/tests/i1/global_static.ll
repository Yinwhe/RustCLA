; ModuleID = '/home/ubuntu/Workspace/RustCLA/ABIChecker/tests/i1/target/debug/build/libloading-f457a757c31f4e2b/out/src/os/unix/global_static.o'
source_filename = "src/os/unix/global_static.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%union.pthread_mutex_t = type { %struct.__pthread_mutex_s }
%struct.__pthread_mutex_s = type { i32, i32, i32, i32, i32, i16, i16, %struct.__pthread_internal_list }
%struct.__pthread_internal_list = type { %struct.__pthread_internal_list*, %struct.__pthread_internal_list* }

@rust_libloading_dlerror_mutex = weak global %union.pthread_mutex_t zeroinitializer, align 8, !dbg !0

; Function Attrs: noinline nounwind optnone uwtable
define weak void @rust_libloading_dlerror_mutex_lock() #0 !dbg !45 {
  %1 = call i32 @pthread_mutex_lock(%union.pthread_mutex_t* noundef @rust_libloading_dlerror_mutex) #3, !dbg !49
  %2 = icmp ne i32 %1, 0, !dbg !51
  br i1 %2, label %3, label %4, !dbg !52

3:                                                ; preds = %0
  call void @abort() #4, !dbg !53
  unreachable, !dbg !53

4:                                                ; preds = %0
  ret void, !dbg !55
}

; Function Attrs: nounwind
declare i32 @pthread_mutex_lock(%union.pthread_mutex_t* noundef) #1

; Function Attrs: noreturn nounwind
declare void @abort() #2

; Function Attrs: noinline nounwind optnone uwtable
define weak void @rust_libloading_dlerror_mutex_unlock() #0 !dbg !56 {
  %1 = call i32 @pthread_mutex_unlock(%union.pthread_mutex_t* noundef @rust_libloading_dlerror_mutex) #3, !dbg !57
  %2 = icmp ne i32 %1, 0, !dbg !59
  br i1 %2, label %3, label %4, !dbg !60

3:                                                ; preds = %0
  call void @abort() #4, !dbg !61
  unreachable, !dbg !61

4:                                                ; preds = %0
  ret void, !dbg !63
}

; Function Attrs: nounwind
declare i32 @pthread_mutex_unlock(%union.pthread_mutex_t* noundef) #1

attributes #0 = { noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nounwind "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { noreturn nounwind "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { nounwind }
attributes #4 = { noreturn nounwind }

!llvm.dbg.cu = !{!2}
!llvm.module.flags = !{!38, !39, !40, !41, !42, !43}
!llvm.ident = !{!44}

!0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
!1 = distinct !DIGlobalVariable(name: "rust_libloading_dlerror_mutex", scope: !2, file: !3, line: 4, type: !5, isLocal: false, isDefinition: true)
!2 = distinct !DICompileUnit(language: DW_LANG_C99, file: !3, producer: "Ubuntu clang version 14.0.0-1ubuntu1.1", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, globals: !4, splitDebugInlining: false, nameTableKind: None)
!3 = !DIFile(filename: "src/os/unix/global_static.c", directory: "/home/ubuntu/.cargo/registry/src/mirrors.ustc.edu.cn-61ef6e0cd06fb9b8/libloading-0.5.2")
!4 = !{!0}
!5 = !DIDerivedType(tag: DW_TAG_typedef, name: "pthread_mutex_t", file: !6, line: 72, baseType: !7)
!6 = !DIFile(filename: "/usr/include/x86_64-linux-gnu/bits/pthreadtypes.h", directory: "")
!7 = distinct !DICompositeType(tag: DW_TAG_union_type, file: !6, line: 67, size: 320, elements: !8)
!8 = !{!9, !31, !36}
!9 = !DIDerivedType(tag: DW_TAG_member, name: "__data", scope: !7, file: !6, line: 69, baseType: !10, size: 320)
!10 = distinct !DICompositeType(tag: DW_TAG_structure_type, name: "__pthread_mutex_s", file: !11, line: 22, size: 320, elements: !12)
!11 = !DIFile(filename: "/usr/include/x86_64-linux-gnu/bits/struct_mutex.h", directory: "")
!12 = !{!13, !15, !17, !18, !19, !20, !22, !23}
!13 = !DIDerivedType(tag: DW_TAG_member, name: "__lock", scope: !10, file: !11, line: 24, baseType: !14, size: 32)
!14 = !DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)
!15 = !DIDerivedType(tag: DW_TAG_member, name: "__count", scope: !10, file: !11, line: 25, baseType: !16, size: 32, offset: 32)
!16 = !DIBasicType(name: "unsigned int", size: 32, encoding: DW_ATE_unsigned)
!17 = !DIDerivedType(tag: DW_TAG_member, name: "__owner", scope: !10, file: !11, line: 26, baseType: !14, size: 32, offset: 64)
!18 = !DIDerivedType(tag: DW_TAG_member, name: "__nusers", scope: !10, file: !11, line: 28, baseType: !16, size: 32, offset: 96)
!19 = !DIDerivedType(tag: DW_TAG_member, name: "__kind", scope: !10, file: !11, line: 32, baseType: !14, size: 32, offset: 128)
!20 = !DIDerivedType(tag: DW_TAG_member, name: "__spins", scope: !10, file: !11, line: 34, baseType: !21, size: 16, offset: 160)
!21 = !DIBasicType(name: "short", size: 16, encoding: DW_ATE_signed)
!22 = !DIDerivedType(tag: DW_TAG_member, name: "__elision", scope: !10, file: !11, line: 35, baseType: !21, size: 16, offset: 176)
!23 = !DIDerivedType(tag: DW_TAG_member, name: "__list", scope: !10, file: !11, line: 36, baseType: !24, size: 128, offset: 192)
!24 = !DIDerivedType(tag: DW_TAG_typedef, name: "__pthread_list_t", file: !25, line: 55, baseType: !26)
!25 = !DIFile(filename: "/usr/include/x86_64-linux-gnu/bits/thread-shared-types.h", directory: "")
!26 = distinct !DICompositeType(tag: DW_TAG_structure_type, name: "__pthread_internal_list", file: !25, line: 51, size: 128, elements: !27)
!27 = !{!28, !30}
!28 = !DIDerivedType(tag: DW_TAG_member, name: "__prev", scope: !26, file: !25, line: 53, baseType: !29, size: 64)
!29 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !26, size: 64)
!30 = !DIDerivedType(tag: DW_TAG_member, name: "__next", scope: !26, file: !25, line: 54, baseType: !29, size: 64, offset: 64)
!31 = !DIDerivedType(tag: DW_TAG_member, name: "__size", scope: !7, file: !6, line: 70, baseType: !32, size: 320)
!32 = !DICompositeType(tag: DW_TAG_array_type, baseType: !33, size: 320, elements: !34)
!33 = !DIBasicType(name: "char", size: 8, encoding: DW_ATE_signed_char)
!34 = !{!35}
!35 = !DISubrange(count: 40)
!36 = !DIDerivedType(tag: DW_TAG_member, name: "__align", scope: !7, file: !6, line: 71, baseType: !37, size: 64)
!37 = !DIBasicType(name: "long", size: 64, encoding: DW_ATE_signed)
!38 = !{i32 7, !"Dwarf Version", i32 4}
!39 = !{i32 2, !"Debug Info Version", i32 3}
!40 = !{i32 1, !"wchar_size", i32 4}
!41 = !{i32 7, !"PIC Level", i32 2}
!42 = !{i32 7, !"uwtable", i32 1}
!43 = !{i32 7, !"frame-pointer", i32 2}
!44 = !{!"Ubuntu clang version 14.0.0-1ubuntu1.1"}
!45 = distinct !DISubprogram(name: "rust_libloading_dlerror_mutex_lock", scope: !3, file: !3, line: 7, type: !46, scopeLine: 8, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !48)
!46 = !DISubroutineType(types: !47)
!47 = !{null}
!48 = !{}
!49 = !DILocation(line: 9, column: 9, scope: !50)
!50 = distinct !DILexicalBlock(scope: !45, file: !3, line: 9, column: 9)
!51 = !DILocation(line: 9, column: 60, scope: !50)
!52 = !DILocation(line: 9, column: 9, scope: !45)
!53 = !DILocation(line: 10, column: 9, scope: !54)
!54 = distinct !DILexicalBlock(scope: !50, file: !3, line: 9, column: 66)
!55 = !DILocation(line: 12, column: 1, scope: !45)
!56 = distinct !DISubprogram(name: "rust_libloading_dlerror_mutex_unlock", scope: !3, file: !3, line: 15, type: !46, scopeLine: 16, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !2, retainedNodes: !48)
!57 = !DILocation(line: 17, column: 9, scope: !58)
!58 = distinct !DILexicalBlock(scope: !56, file: !3, line: 17, column: 9)
!59 = !DILocation(line: 17, column: 62, scope: !58)
!60 = !DILocation(line: 17, column: 9, scope: !56)
!61 = !DILocation(line: 18, column: 9, scope: !62)
!62 = distinct !DILexicalBlock(scope: !58, file: !3, line: 17, column: 68)
!63 = !DILocation(line: 20, column: 1, scope: !56)
