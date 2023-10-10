; ModuleID = '/home/ubuntu/Workspace/RustCLA/ABIChecker/tests/wstest/target/debug/build/sys-1eb5e4cf3be897ed/out/csrc/hello.o'
source_filename = "csrc/hello.cpp"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%class.Hello = type { [16 x i8], i8* }

@.str = private unnamed_addr constant [13 x i8] c"Hello World!\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1

@_ZN5HelloC1Ev = unnamed_addr alias void (%class.Hello*), void (%class.Hello*)* @_ZN5HelloC2Ev
@_ZN5HelloD1Ev = unnamed_addr alias void (%class.Hello*), void (%class.Hello*)* @_ZN5HelloD2Ev

; Function Attrs: noinline nounwind optnone uwtable
define void @_ZN5HelloC2Ev(%class.Hello* noundef nonnull align 8 dereferenceable(24) %0) unnamed_addr #0 align 2 !dbg !25 {
  %2 = alloca %class.Hello*, align 8
  store %class.Hello* %0, %class.Hello** %2, align 8
  call void @llvm.dbg.declare(metadata %class.Hello** %2, metadata !27, metadata !DIExpression()), !dbg !29
  %3 = load %class.Hello*, %class.Hello** %2, align 8
  %4 = call noalias i8* @malloc(i64 noundef 16) #5, !dbg !30
  %5 = getelementptr inbounds %class.Hello, %class.Hello* %3, i32 0, i32 1, !dbg !32
  store i8* %4, i8** %5, align 8, !dbg !33
  %6 = getelementptr inbounds %class.Hello, %class.Hello* %3, i32 0, i32 0, !dbg !34
  %7 = getelementptr inbounds [16 x i8], [16 x i8]* %6, i64 0, i64 0, !dbg !34
  %8 = call i8* @strcpy(i8* noundef %7, i8* noundef getelementptr inbounds ([13 x i8], [13 x i8]* @.str, i64 0, i64 0)) #5, !dbg !35
  ret void, !dbg !36
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

; Function Attrs: nounwind
declare noalias i8* @malloc(i64 noundef) #2

; Function Attrs: nounwind
declare i8* @strcpy(i8* noundef, i8* noundef) #2

; Function Attrs: noinline nounwind optnone uwtable
define void @_ZN5HelloD2Ev(%class.Hello* noundef nonnull align 8 dereferenceable(24) %0) unnamed_addr #0 align 2 !dbg !37 {
  %2 = alloca %class.Hello*, align 8
  store %class.Hello* %0, %class.Hello** %2, align 8
  call void @llvm.dbg.declare(metadata %class.Hello** %2, metadata !38, metadata !DIExpression()), !dbg !39
  %3 = load %class.Hello*, %class.Hello** %2, align 8
  %4 = getelementptr inbounds %class.Hello, %class.Hello* %3, i32 0, i32 1, !dbg !40
  %5 = load i8*, i8** %4, align 8, !dbg !40
  call void @free(i8* noundef %5) #5, !dbg !42
  ret void, !dbg !43
}

; Function Attrs: nounwind
declare void @free(i8* noundef) #2

; Function Attrs: mustprogress noinline optnone uwtable
define void @_ZN5Hello8sayHelloEv(%class.Hello* noundef nonnull align 8 dereferenceable(24) %0) #3 align 2 !dbg !44 {
  %2 = alloca %class.Hello*, align 8
  store %class.Hello* %0, %class.Hello** %2, align 8
  call void @llvm.dbg.declare(metadata %class.Hello** %2, metadata !45, metadata !DIExpression()), !dbg !46
  %3 = load %class.Hello*, %class.Hello** %2, align 8
  %4 = getelementptr inbounds %class.Hello, %class.Hello* %3, i32 0, i32 0, !dbg !47
  %5 = getelementptr inbounds [16 x i8], [16 x i8]* %4, i64 0, i64 0, !dbg !47
  %6 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([4 x i8], [4 x i8]* @.str.1, i64 0, i64 0), i8* noundef %5), !dbg !48
  ret void, !dbg !49
}

declare i32 @printf(i8* noundef, ...) #4

attributes #0 = { noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #2 = { nounwind "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { mustprogress noinline optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #4 = { "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #5 = { nounwind }

!llvm.dbg.cu = !{!0}
!llvm.module.flags = !{!18, !19, !20, !21, !22, !23}
!llvm.ident = !{!24}

!0 = distinct !DICompileUnit(language: DW_LANG_C_plus_plus_14, file: !1, producer: "Ubuntu clang version 14.0.0-1ubuntu1.1", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, retainedTypes: !2, splitDebugInlining: false, nameTableKind: None)
!1 = !DIFile(filename: "csrc/hello.cpp", directory: "/home/ubuntu/Workspace/RustCLA/ABIChecker/tests/wstest/sys")
!2 = !{!3}
!3 = distinct !DICompositeType(tag: DW_TAG_class_type, name: "Hello", file: !1, line: 5, size: 192, flags: DIFlagTypePassByReference | DIFlagNonTrivial, elements: !4, identifier: "_ZTS5Hello")
!4 = !{!5, !10, !12, !16, !17}
!5 = !DIDerivedType(tag: DW_TAG_member, name: "data", scope: !3, file: !1, line: 7, baseType: !6, size: 128)
!6 = !DICompositeType(tag: DW_TAG_array_type, baseType: !7, size: 128, elements: !8)
!7 = !DIBasicType(name: "char", size: 8, encoding: DW_ATE_signed_char)
!8 = !{!9}
!9 = !DISubrange(count: 16)
!10 = !DIDerivedType(tag: DW_TAG_member, name: "ptr", scope: !3, file: !1, line: 8, baseType: !11, size: 64, offset: 128)
!11 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: null, size: 64)
!12 = !DISubprogram(name: "Hello", scope: !3, file: !1, line: 10, type: !13, scopeLine: 10, flags: DIFlagPublic | DIFlagPrototyped, spFlags: 0)
!13 = !DISubroutineType(types: !14)
!14 = !{null, !15}
!15 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !3, size: 64, flags: DIFlagArtificial | DIFlagObjectPointer)
!16 = !DISubprogram(name: "~Hello", scope: !3, file: !1, line: 11, type: !13, scopeLine: 11, flags: DIFlagPublic | DIFlagPrototyped, spFlags: 0)
!17 = !DISubprogram(name: "sayHello", linkageName: "_ZN5Hello8sayHelloEv", scope: !3, file: !1, line: 12, type: !13, scopeLine: 12, flags: DIFlagPublic | DIFlagPrototyped, spFlags: 0)
!18 = !{i32 7, !"Dwarf Version", i32 4}
!19 = !{i32 2, !"Debug Info Version", i32 3}
!20 = !{i32 1, !"wchar_size", i32 4}
!21 = !{i32 7, !"PIC Level", i32 2}
!22 = !{i32 7, !"uwtable", i32 1}
!23 = !{i32 7, !"frame-pointer", i32 2}
!24 = !{!"Ubuntu clang version 14.0.0-1ubuntu1.1"}
!25 = distinct !DISubprogram(name: "Hello", linkageName: "_ZN5HelloC2Ev", scope: !3, file: !1, line: 15, type: !13, scopeLine: 15, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0, declaration: !12, retainedNodes: !26)
!26 = !{}
!27 = !DILocalVariable(name: "this", arg: 1, scope: !25, type: !28, flags: DIFlagArtificial | DIFlagObjectPointer)
!28 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !3, size: 64)
!29 = !DILocation(line: 0, scope: !25)
!30 = !DILocation(line: 16, column: 11, scope: !31)
!31 = distinct !DILexicalBlock(scope: !25, file: !1, line: 15, column: 16)
!32 = !DILocation(line: 16, column: 5, scope: !31)
!33 = !DILocation(line: 16, column: 9, scope: !31)
!34 = !DILocation(line: 17, column: 12, scope: !31)
!35 = !DILocation(line: 17, column: 5, scope: !31)
!36 = !DILocation(line: 18, column: 1, scope: !25)
!37 = distinct !DISubprogram(name: "~Hello", linkageName: "_ZN5HelloD2Ev", scope: !3, file: !1, line: 20, type: !13, scopeLine: 20, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0, declaration: !16, retainedNodes: !26)
!38 = !DILocalVariable(name: "this", arg: 1, scope: !37, type: !28, flags: DIFlagArtificial | DIFlagObjectPointer)
!39 = !DILocation(line: 0, scope: !37)
!40 = !DILocation(line: 21, column: 10, scope: !41)
!41 = distinct !DILexicalBlock(scope: !37, file: !1, line: 20, column: 17)
!42 = !DILocation(line: 21, column: 5, scope: !41)
!43 = !DILocation(line: 22, column: 1, scope: !37)
!44 = distinct !DISubprogram(name: "sayHello", linkageName: "_ZN5Hello8sayHelloEv", scope: !3, file: !1, line: 24, type: !13, scopeLine: 24, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !0, declaration: !17, retainedNodes: !26)
!45 = !DILocalVariable(name: "this", arg: 1, scope: !44, type: !28, flags: DIFlagArtificial | DIFlagObjectPointer)
!46 = !DILocation(line: 0, scope: !44)
!47 = !DILocation(line: 25, column: 20, scope: !44)
!48 = !DILocation(line: 25, column: 5, scope: !44)
!49 = !DILocation(line: 26, column: 1, scope: !44)
