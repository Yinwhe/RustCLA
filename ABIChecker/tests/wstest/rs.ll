; ModuleID = '/home/ubuntu/Workspace/RustCLA/ABIChecker/tests/wstest/target/debug/deps/rs-047a6509641c6b93.bc'
source_filename = "19ymfwff45voj95"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"sys::Hello" = type { [16 x i8], i8* }

; Function Attrs: nonlazybind uwtable
define void @_ZN2rs9get_hello17h66af72a359f7ba77E(%"sys::Hello"* sret(%"sys::Hello") %0) unnamed_addr #0 !dbg !15 {
start:
  call void @_ZN3sys5Hello3new17h3c7bea36bb2a3dc0E(%"sys::Hello"* sret(%"sys::Hello") %0), !dbg !31
  br label %bb1, !dbg !31

bb1:                                              ; preds = %start
  ret void, !dbg !32
}

; Function Attrs: nonlazybind uwtable
declare void @_ZN3sys5Hello3new17h3c7bea36bb2a3dc0E(%"sys::Hello"* sret(%"sys::Hello")) unnamed_addr #0

attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.dbg.cu = !{!4}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{i32 2, !"Dwarf Version", i32 4}
!3 = !{i32 2, !"Debug Info Version", i32 3}
!4 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !5, producer: "clang LLVM (rustc version 1.64.0-nightly (f9cba6374 2022-07-31))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !6)
!5 = !DIFile(filename: "rs/src/lib.rs/@/19ymfwff45voj95", directory: "/home/ubuntu/Workspace/RustCLA/ABIChecker/tests/wstest")
!6 = !{!7}
!7 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "c_void", scope: !9, file: !8, baseType: !11, size: 8, align: 8, flags: DIFlagEnumClass, elements: !12)
!8 = !DIFile(filename: "<unknown>", directory: "")
!9 = !DINamespace(name: "ffi", scope: !10)
!10 = !DINamespace(name: "core", scope: null)
!11 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!12 = !{!13, !14}
!13 = !DIEnumerator(name: "__variant1", value: 0, isUnsigned: true)
!14 = !DIEnumerator(name: "__variant2", value: 1, isUnsigned: true)
!15 = distinct !DISubprogram(name: "get_hello", linkageName: "_ZN2rs9get_hello17h66af72a359f7ba77E", scope: !17, file: !16, line: 3, type: !18, scopeLine: 3, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !4, templateParams: !30, retainedNodes: !30)
!16 = !DIFile(filename: "rs/src/lib.rs", directory: "/home/ubuntu/Workspace/RustCLA/ABIChecker/tests/wstest", checksumkind: CSK_MD5, checksum: "b16ecb01903838f0b1f7ca315269e7e6")
!17 = !DINamespace(name: "rs", scope: null)
!18 = !DISubroutineType(types: !19)
!19 = !{!20}
!20 = !DICompositeType(tag: DW_TAG_structure_type, name: "Hello", scope: !21, file: !8, size: 192, align: 64, elements: !22, templateParams: !30, identifier: "c8eddfd74cbee4a9b0862492ef37f0b1")
!21 = !DINamespace(name: "sys", scope: null)
!22 = !{!23, !28}
!23 = !DIDerivedType(tag: DW_TAG_member, name: "data", scope: !20, file: !8, baseType: !24, size: 128, align: 8)
!24 = !DICompositeType(tag: DW_TAG_array_type, baseType: !25, size: 128, align: 8, elements: !26)
!25 = !DIBasicType(name: "i8", size: 8, encoding: DW_ATE_signed)
!26 = !{!27}
!27 = !DISubrange(count: 16, lowerBound: 0)
!28 = !DIDerivedType(tag: DW_TAG_member, name: "ptr", scope: !20, file: !8, baseType: !29, size: 64, align: 64, offset: 128)
!29 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut core::ffi::c_void", baseType: !7, size: 64, align: 64, dwarfAddressSpace: 0)
!30 = !{}
!31 = !DILocation(line: 4, column: 14, scope: !15)
!32 = !DILocation(line: 5, column: 2, scope: !15)
