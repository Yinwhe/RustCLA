; ModuleID = '/home/ubuntu/Workspace/RustCLA/ABIChecker/tests/i1/target/debug/build/issue1s-afd9304099239144/out/src/hello.o'
source_filename = "src/hello.cpp"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%class.A = type <{ i8*, i32, [4 x i8] }>
%class.B = type { %class.A.base, i32 }
%class.A.base = type <{ i8*, i32 }>

@.str = private unnamed_addr constant [14 x i8] c"Hello, world!\00", align 1

@_ZN1AC1Ev = unnamed_addr alias void (%class.A*), void (%class.A*)* @_ZN1AC2Ev
@_ZN1BC1Ev = unnamed_addr alias void (%class.B*), void (%class.B*)* @_ZN1BC2Ev

; Function Attrs: noinline nounwind optnone uwtable
define void @_ZN1AC2Ev(%class.A* noundef nonnull align 8 dereferenceable(12) %0) unnamed_addr #0 align 2 !dbg !39 {
  %2 = alloca %class.A*, align 8
  store %class.A* %0, %class.A** %2, align 8
  call void @llvm.dbg.declare(metadata %class.A** %2, metadata !41, metadata !DIExpression()), !dbg !43
  %3 = load %class.A*, %class.A** %2, align 8
  %4 = getelementptr inbounds %class.A, %class.A* %3, i32 0, i32 0, !dbg !44
  store i8* null, i8** %4, align 8, !dbg !46
  %5 = getelementptr inbounds %class.A, %class.A* %3, i32 0, i32 1, !dbg !47
  store i32 0, i32* %5, align 8, !dbg !48
  ret void, !dbg !49
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

; Function Attrs: noinline nounwind optnone uwtable
define void @_ZN1BC2Ev(%class.B* noundef nonnull align 8 dereferenceable(16) %0) unnamed_addr #0 align 2 !dbg !50 {
  %2 = alloca %class.B*, align 8
  store %class.B* %0, %class.B** %2, align 8
  call void @llvm.dbg.declare(metadata %class.B** %2, metadata !51, metadata !DIExpression()), !dbg !53
  %3 = load %class.B*, %class.B** %2, align 8
  %4 = bitcast %class.B* %3 to %class.A*, !dbg !54
  call void @_ZN1AC2Ev(%class.A* noundef nonnull align 8 dereferenceable(12) %4), !dbg !55
  %5 = getelementptr inbounds %class.B, %class.B* %3, i32 0, i32 1, !dbg !56
  store i32 0, i32* %5, align 4, !dbg !58
  ret void, !dbg !59
}

; Function Attrs: mustprogress noinline optnone uwtable
define void @_ZNK1A9say_helloEv(%class.A* noundef nonnull align 8 dereferenceable(12) %0) #2 align 2 !dbg !60 {
  %2 = alloca %class.A*, align 8
  store %class.A* %0, %class.A** %2, align 8
  call void @llvm.dbg.declare(metadata %class.A** %2, metadata !61, metadata !DIExpression()), !dbg !63
  %3 = load %class.A*, %class.A** %2, align 8
  %4 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([14 x i8], [14 x i8]* @.str, i64 0, i64 0)), !dbg !64
  ret void, !dbg !65
}

declare i32 @printf(i8* noundef, ...) #3

; Function Attrs: mustprogress noinline optnone uwtable
define void @_ZNK1B9say_helloEv(%class.B* noundef nonnull align 8 dereferenceable(16) %0) #2 align 2 !dbg !66 {
  %2 = alloca %class.B*, align 8
  store %class.B* %0, %class.B** %2, align 8
  call void @llvm.dbg.declare(metadata %class.B** %2, metadata !67, metadata !DIExpression()), !dbg !69
  %3 = load %class.B*, %class.B** %2, align 8
  %4 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([14 x i8], [14 x i8]* @.str, i64 0, i64 0)), !dbg !70
  ret void, !dbg !71
}

attributes #0 = { noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #2 = { mustprogress noinline optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }

!llvm.dbg.cu = !{!0}
!llvm.module.flags = !{!32, !33, !34, !35, !36, !37}
!llvm.ident = !{!38}

!0 = distinct !DICompileUnit(language: DW_LANG_C_plus_plus_11, file: !1, producer: "Ubuntu clang version 14.0.0-1ubuntu1.1", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, retainedTypes: !2, splitDebugInlining: false, nameTableKind: None)
!1 = !DIFile(filename: "src/hello.cpp", directory: "/home/ubuntu/Workspace/RustCLA/ABIChecker/tests/i1")
!2 = !{!3, !19}
!3 = distinct !DICompositeType(tag: DW_TAG_class_type, name: "A", file: !4, line: 3, size: 128, flags: DIFlagTypePassByValue | DIFlagNonTrivial, elements: !5, identifier: "_ZTS1A")
!4 = !DIFile(filename: "src/../include/hello.hpp", directory: "/home/ubuntu/Workspace/RustCLA/ABIChecker/tests/i1")
!5 = !{!6, !8, !10, !14}
!6 = !DIDerivedType(tag: DW_TAG_member, name: "ptr", scope: !3, file: !4, line: 10, baseType: !7, size: 64)
!7 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: null, size: 64)
!8 = !DIDerivedType(tag: DW_TAG_member, name: "_m_buf", scope: !3, file: !4, line: 11, baseType: !9, size: 32, offset: 64)
!9 = !DIBasicType(name: "int", size: 32, encoding: DW_ATE_signed)
!10 = !DISubprogram(name: "A", scope: !3, file: !4, line: 6, type: !11, scopeLine: 6, flags: DIFlagPublic | DIFlagPrototyped, spFlags: 0)
!11 = !DISubroutineType(types: !12)
!12 = !{null, !13}
!13 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !3, size: 64, flags: DIFlagArtificial | DIFlagObjectPointer)
!14 = !DISubprogram(name: "say_hello", linkageName: "_ZNK1A9say_helloEv", scope: !3, file: !4, line: 7, type: !15, scopeLine: 7, flags: DIFlagPublic | DIFlagPrototyped, spFlags: 0)
!15 = !DISubroutineType(types: !16)
!16 = !{null, !17}
!17 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !18, size: 64, flags: DIFlagArtificial | DIFlagObjectPointer)
!18 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !3)
!19 = distinct !DICompositeType(tag: DW_TAG_class_type, name: "B", file: !4, line: 14, size: 128, flags: DIFlagTypePassByValue | DIFlagNonTrivial, elements: !20, identifier: "_ZTS1B")
!20 = !{!21, !22, !23, !27}
!21 = !DIDerivedType(tag: DW_TAG_inheritance, scope: !19, baseType: !3, extraData: i32 0)
!22 = !DIDerivedType(tag: DW_TAG_member, name: "_m_val", scope: !19, file: !4, line: 21, baseType: !9, size: 32, offset: 96)
!23 = !DISubprogram(name: "B", scope: !19, file: !4, line: 17, type: !24, scopeLine: 17, flags: DIFlagPublic | DIFlagPrototyped, spFlags: 0)
!24 = !DISubroutineType(types: !25)
!25 = !{null, !26}
!26 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !19, size: 64, flags: DIFlagArtificial | DIFlagObjectPointer)
!27 = !DISubprogram(name: "say_hello", linkageName: "_ZNK1B9say_helloEv", scope: !19, file: !4, line: 18, type: !28, scopeLine: 18, flags: DIFlagPublic | DIFlagPrototyped, spFlags: 0)
!28 = !DISubroutineType(types: !29)
!29 = !{null, !30}
!30 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !31, size: 64, flags: DIFlagArtificial | DIFlagObjectPointer)
!31 = !DIDerivedType(tag: DW_TAG_const_type, baseType: !19)
!32 = !{i32 7, !"Dwarf Version", i32 4}
!33 = !{i32 2, !"Debug Info Version", i32 3}
!34 = !{i32 1, !"wchar_size", i32 4}
!35 = !{i32 7, !"PIC Level", i32 2}
!36 = !{i32 7, !"uwtable", i32 1}
!37 = !{i32 7, !"frame-pointer", i32 2}
!38 = !{!"Ubuntu clang version 14.0.0-1ubuntu1.1"}
!39 = distinct !DISubprogram(name: "A", linkageName: "_ZN1AC2Ev", scope: !3, file: !1, line: 4, type: !11, scopeLine: 5, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0, declaration: !10, retainedNodes: !40)
!40 = !{}
!41 = !DILocalVariable(name: "this", arg: 1, scope: !39, type: !42, flags: DIFlagArtificial | DIFlagObjectPointer)
!42 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !3, size: 64)
!43 = !DILocation(line: 0, scope: !39)
!44 = !DILocation(line: 6, column: 5, scope: !45)
!45 = distinct !DILexicalBlock(scope: !39, file: !1, line: 5, column: 1)
!46 = !DILocation(line: 6, column: 9, scope: !45)
!47 = !DILocation(line: 7, column: 5, scope: !45)
!48 = !DILocation(line: 7, column: 12, scope: !45)
!49 = !DILocation(line: 8, column: 1, scope: !39)
!50 = distinct !DISubprogram(name: "B", linkageName: "_ZN1BC2Ev", scope: !19, file: !1, line: 10, type: !24, scopeLine: 11, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0, declaration: !23, retainedNodes: !40)
!51 = !DILocalVariable(name: "this", arg: 1, scope: !50, type: !52, flags: DIFlagArtificial | DIFlagObjectPointer)
!52 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !19, size: 64)
!53 = !DILocation(line: 0, scope: !50)
!54 = !DILocation(line: 11, column: 1, scope: !50)
!55 = !DILocation(line: 10, column: 4, scope: !50)
!56 = !DILocation(line: 12, column: 5, scope: !57)
!57 = distinct !DILexicalBlock(scope: !50, file: !1, line: 11, column: 1)
!58 = !DILocation(line: 12, column: 12, scope: !57)
!59 = !DILocation(line: 13, column: 1, scope: !50)
!60 = distinct !DISubprogram(name: "say_hello", linkageName: "_ZNK1A9say_helloEv", scope: !3, file: !1, line: 15, type: !15, scopeLine: 15, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0, declaration: !14, retainedNodes: !40)
!61 = !DILocalVariable(name: "this", arg: 1, scope: !60, type: !62, flags: DIFlagArtificial | DIFlagObjectPointer)
!62 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !18, size: 64)
!63 = !DILocation(line: 0, scope: !60)
!64 = !DILocation(line: 16, column: 5, scope: !60)
!65 = !DILocation(line: 17, column: 1, scope: !60)
!66 = distinct !DISubprogram(name: "say_hello", linkageName: "_ZNK1B9say_helloEv", scope: !19, file: !1, line: 19, type: !28, scopeLine: 19, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0, declaration: !27, retainedNodes: !40)
!67 = !DILocalVariable(name: "this", arg: 1, scope: !66, type: !68, flags: DIFlagArtificial | DIFlagObjectPointer)
!68 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !31, size: 64)
!69 = !DILocation(line: 0, scope: !66)
!70 = !DILocation(line: 20, column: 5, scope: !66)
!71 = !DILocation(line: 21, column: 1, scope: !66)
