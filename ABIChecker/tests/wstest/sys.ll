; ModuleID = '/home/ubuntu/Workspace/RustCLA/ABIChecker/tests/wstest/target/debug/deps/sys-4ef91f35044af93c.bc'
source_filename = "2ph5xhx6fjxfiqym"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"core::fmt::Formatter" = type { { i64, i64 }, { i64, i64 }, { {}*, [3 x i64]* }, i32, i32, i8, [7 x i8] }
%"core::fmt::builders::DebugList" = type { %"core::fmt::builders::DebugInner" }
%"core::fmt::builders::DebugInner" = type { %"core::fmt::Formatter"*, i8, i8, [6 x i8] }
%"core::ops::range::RangeFull" = type {}
%"core::ptr::metadata::PtrComponents<u8>" = type { {}*, {} }
%"core::ptr::metadata::PtrRepr<u8>" = type { [1 x i64] }
%Hello = type { [16 x i8], i8* }
%"core::mem::manually_drop::ManuallyDrop<Hello>" = type { %Hello }
%"core::mem::maybe_uninit::MaybeUninit<Hello>" = type { [3 x i64] }
%"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [6 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

@vtable.0 = private unnamed_addr constant <{ i8*, [16 x i8], i8* }> <{ i8* bitcast (void (i8**)* @"_ZN4core3ptr27drop_in_place$LT$$RF$i8$GT$17he47ffb12beaadcd1E" to i8*), [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", i8* bitcast (i1 (i8**, %"core::fmt::Formatter"*)* @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h9d33819da977a53aE" to i8*) }>, align 8, !dbg !0
@alloc41 = private unnamed_addr constant <{ [27 x i8] }> <{ [27 x i8] c"sys/src/../csrc/bindings.rs" }>, align 1
@alloc42 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [27 x i8] }>, <{ [27 x i8] }>* @alloc41, i32 0, i32 0, i32 0), [16 x i8] c"\1B\00\00\00\00\00\00\00\1C\00\00\00\17\00\00\00" }>, align 8
@alloc43 = private unnamed_addr constant <{ [5 x i8] }> <{ [5 x i8] c"Hello" }>, align 1
@alloc44 = private unnamed_addr constant <{ [4 x i8] }> <{ [4 x i8] c"data" }>, align 1
@vtable.1 = private unnamed_addr constant <{ i8*, [16 x i8], i8* }> <{ i8* bitcast (void ([16 x i8]**)* @"_ZN4core3ptr49drop_in_place$LT$$RF$$u5b$i8$u3b$$u20$16$u5d$$GT$17h1b4d6bf73e0f167eE" to i8*), [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", i8* bitcast (i1 ([16 x i8]**, %"core::fmt::Formatter"*)* @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hba762ccd28fafa81E" to i8*) }>, align 8, !dbg !15
@alloc48 = private unnamed_addr constant <{ [3 x i8] }> <{ [3 x i8] c"ptr" }>, align 1
@vtable.2 = private unnamed_addr constant <{ i8*, [16 x i8], i8* }> <{ i8* bitcast (void (i8***)* @"_ZN4core3ptr54drop_in_place$LT$$RF$$BP$mut$u20$core..ffi..c_void$GT$17hb669b5b1cf12a613E" to i8*), [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", i8* bitcast (i1 (i8***, %"core::fmt::Formatter"*)* @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h2eeb052d2a18da27E" to i8*) }>, align 8, !dbg !27

; Function Attrs: nonlazybind uwtable
define zeroext i1 @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h2eeb052d2a18da27E"(i8*** align 8 %self, %"core::fmt::Formatter"* align 8 %f) unnamed_addr #0 !dbg !61 {
start:
  %f.dbg.spill = alloca %"core::fmt::Formatter"*, align 8
  %self.dbg.spill = alloca i8***, align 8
  store i8*** %self, i8**** %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i8**** %self.dbg.spill, metadata !122, metadata !DIExpression()), !dbg !126
  store %"core::fmt::Formatter"* %f, %"core::fmt::Formatter"** %f.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata %"core::fmt::Formatter"** %f.dbg.spill, metadata !123, metadata !DIExpression()), !dbg !127
  %_6 = load i8**, i8*** %self, align 8, !dbg !128, !nonnull !14, !align !129, !noundef !14
  %0 = call zeroext i1 @"_ZN50_$LT$$BP$mut$u20$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h8c2be5a5f4e50655E"(i8** align 8 %_6, %"core::fmt::Formatter"* align 8 %f), !dbg !130
  br label %bb1, !dbg !130

bb1:                                              ; preds = %start
  ret i1 %0, !dbg !131
}

; Function Attrs: nonlazybind uwtable
define zeroext i1 @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h5311ceffab2d7116E"({ [0 x i8]*, i64 }* align 8 %self, %"core::fmt::Formatter"* align 8 %f) unnamed_addr #0 !dbg !132 {
start:
  %f.dbg.spill = alloca %"core::fmt::Formatter"*, align 8
  %self.dbg.spill = alloca { [0 x i8]*, i64 }*, align 8
  store { [0 x i8]*, i64 }* %self, { [0 x i8]*, i64 }** %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata { [0 x i8]*, i64 }** %self.dbg.spill, metadata !142, metadata !DIExpression()), !dbg !146
  store %"core::fmt::Formatter"* %f, %"core::fmt::Formatter"** %f.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata %"core::fmt::Formatter"** %f.dbg.spill, metadata !143, metadata !DIExpression()), !dbg !147
  %0 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %self, i32 0, i32 0, !dbg !148
  %_6.0 = load [0 x i8]*, [0 x i8]** %0, align 8, !dbg !148, !nonnull !14, !align !149, !noundef !14
  %1 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %self, i32 0, i32 1, !dbg !148
  %_6.1 = load i64, i64* %1, align 8, !dbg !148
  %2 = call zeroext i1 @"_ZN48_$LT$$u5b$T$u5d$$u20$as$u20$core..fmt..Debug$GT$3fmt17hbe5f57de97c67132E"([0 x i8]* align 1 %_6.0, i64 %_6.1, %"core::fmt::Formatter"* align 8 %f), !dbg !150
  br label %bb1, !dbg !150

bb1:                                              ; preds = %start
  ret i1 %2, !dbg !151
}

; Function Attrs: nonlazybind uwtable
define zeroext i1 @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h9d33819da977a53aE"(i8** align 8 %self, %"core::fmt::Formatter"* align 8 %f) unnamed_addr #0 !dbg !152 {
start:
  %f.dbg.spill = alloca %"core::fmt::Formatter"*, align 8
  %self.dbg.spill = alloca i8**, align 8
  store i8** %self, i8*** %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i8*** %self.dbg.spill, metadata !157, metadata !DIExpression()), !dbg !159
  store %"core::fmt::Formatter"* %f, %"core::fmt::Formatter"** %f.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata %"core::fmt::Formatter"** %f.dbg.spill, metadata !158, metadata !DIExpression()), !dbg !160
  %_6 = load i8*, i8** %self, align 8, !dbg !161, !nonnull !14, !align !149, !noundef !14
  %0 = call zeroext i1 @"_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i8$GT$3fmt17h32a029fc41c52f79E"(i8* align 1 %_6, %"core::fmt::Formatter"* align 8 %f), !dbg !162
  br label %bb1, !dbg !162

bb1:                                              ; preds = %start
  ret i1 %0, !dbg !163
}

; Function Attrs: nonlazybind uwtable
define zeroext i1 @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hba762ccd28fafa81E"([16 x i8]** align 8 %self, %"core::fmt::Formatter"* align 8 %f) unnamed_addr #0 !dbg !164 {
start:
  %f.dbg.spill = alloca %"core::fmt::Formatter"*, align 8
  %self.dbg.spill = alloca [16 x i8]**, align 8
  store [16 x i8]** %self, [16 x i8]*** %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata [16 x i8]*** %self.dbg.spill, metadata !169, metadata !DIExpression()), !dbg !173
  store %"core::fmt::Formatter"* %f, %"core::fmt::Formatter"** %f.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata %"core::fmt::Formatter"** %f.dbg.spill, metadata !170, metadata !DIExpression()), !dbg !174
  %_6 = load [16 x i8]*, [16 x i8]** %self, align 8, !dbg !175, !nonnull !14, !align !149, !noundef !14
  %0 = call zeroext i1 @"_ZN4core5array69_$LT$impl$u20$core..fmt..Debug$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3fmt17h90ddc53a53953c41E"([16 x i8]* align 1 %_6, %"core::fmt::Formatter"* align 8 %f), !dbg !176
  br label %bb1, !dbg !176

bb1:                                              ; preds = %start
  ret i1 %0, !dbg !177
}

; Function Attrs: nonlazybind uwtable
define zeroext i1 @"_ZN48_$LT$$u5b$T$u5d$$u20$as$u20$core..fmt..Debug$GT$3fmt17hbe5f57de97c67132E"([0 x i8]* align 1 %self.0, i64 %self.1, %"core::fmt::Formatter"* align 8 %f) unnamed_addr #0 !dbg !178 {
start:
  %self.dbg.spill1 = alloca { [0 x i8]*, i64 }, align 8
  %f.dbg.spill = alloca %"core::fmt::Formatter"*, align 8
  %self.dbg.spill = alloca { [0 x i8]*, i64 }, align 8
  %_6 = alloca %"core::fmt::builders::DebugList", align 8
  %0 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %self.dbg.spill, i32 0, i32 0
  store [0 x i8]* %self.0, [0 x i8]** %0, align 8
  %1 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %self.dbg.spill, i32 0, i32 1
  store i64 %self.1, i64* %1, align 8
  call void @llvm.dbg.declare(metadata { [0 x i8]*, i64 }* %self.dbg.spill, metadata !183, metadata !DIExpression()), !dbg !185
  store %"core::fmt::Formatter"* %f, %"core::fmt::Formatter"** %f.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata %"core::fmt::Formatter"** %f.dbg.spill, metadata !184, metadata !DIExpression()), !dbg !186
  call void @_ZN4core3fmt9Formatter10debug_list17hdc92f4eec9879f86E(%"core::fmt::builders::DebugList"* sret(%"core::fmt::builders::DebugList") %_6, %"core::fmt::Formatter"* align 8 %f), !dbg !187
  br label %bb1, !dbg !187

bb1:                                              ; preds = %start
  %2 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %self.dbg.spill1, i32 0, i32 0, !dbg !188
  store [0 x i8]* %self.0, [0 x i8]** %2, align 8, !dbg !188
  %3 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %self.dbg.spill1, i32 0, i32 1, !dbg !188
  store i64 %self.1, i64* %3, align 8, !dbg !188
  call void @llvm.dbg.declare(metadata { [0 x i8]*, i64 }* %self.dbg.spill1, metadata !189, metadata !DIExpression()), !dbg !214
  %4 = call { i8*, i8* } @"_ZN4core5slice4iter13Iter$LT$T$GT$3new17h34ce25feedef38b4E"([0 x i8]* align 1 %self.0, i64 %self.1), !dbg !214
  %_8.0 = extractvalue { i8*, i8* } %4, 0, !dbg !214
  %_8.1 = extractvalue { i8*, i8* } %4, 1, !dbg !214
  br label %bb4, !dbg !214

bb4:                                              ; preds = %bb1
  %_4 = call align 8 %"core::fmt::builders::DebugList"* @_ZN4core3fmt8builders9DebugList7entries17hf305a3c344867e60E(%"core::fmt::builders::DebugList"* align 8 %_6, i8* %_8.0, i8* %_8.1), !dbg !187
  br label %bb2, !dbg !187

bb2:                                              ; preds = %bb4
  %5 = call zeroext i1 @_ZN4core3fmt8builders9DebugList6finish17h6ba6f640472a04e8E(%"core::fmt::builders::DebugList"* align 8 %_4), !dbg !187
  br label %bb3, !dbg !187

bb3:                                              ; preds = %bb2
  ret i1 %5, !dbg !215
}

; Function Attrs: inlinehint nonlazybind uwtable
define internal zeroext i1 @"_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i8$GT$3fmt17h32a029fc41c52f79E"(i8* align 1 %self, %"core::fmt::Formatter"* align 8 %f) unnamed_addr #1 !dbg !216 {
start:
  %f.dbg.spill = alloca %"core::fmt::Formatter"*, align 8
  %self.dbg.spill = alloca i8*, align 8
  %0 = alloca i8, align 1
  store i8* %self, i8** %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill, metadata !223, metadata !DIExpression()), !dbg !225
  store %"core::fmt::Formatter"* %f, %"core::fmt::Formatter"** %f.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata %"core::fmt::Formatter"** %f.dbg.spill, metadata !224, metadata !DIExpression()), !dbg !226
  %_3 = call zeroext i1 @_ZN4core3fmt9Formatter15debug_lower_hex17h0be447fd1ce6c68cE(%"core::fmt::Formatter"* align 8 %f), !dbg !227
  br label %bb1, !dbg !227

bb1:                                              ; preds = %start
  br i1 %_3, label %bb2, label %bb4, !dbg !227

bb4:                                              ; preds = %bb1
  %_7 = call zeroext i1 @_ZN4core3fmt9Formatter15debug_upper_hex17he97a574bdc69fc4aE(%"core::fmt::Formatter"* align 8 %f), !dbg !228
  br label %bb5, !dbg !228

bb2:                                              ; preds = %bb1
  %1 = call zeroext i1 @"_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i8$GT$3fmt17hb40b1edf2ea57794E"(i8* align 1 %self, %"core::fmt::Formatter"* align 8 %f), !dbg !229
  %2 = zext i1 %1 to i8, !dbg !229
  store i8 %2, i8* %0, align 1, !dbg !229
  br label %bb3, !dbg !229

bb3:                                              ; preds = %bb2
  br label %bb11, !dbg !230

bb11:                                             ; preds = %bb10, %bb3
  %3 = load i8, i8* %0, align 1, !dbg !231, !range !232, !noundef !14
  %4 = trunc i8 %3 to i1, !dbg !231
  ret i1 %4, !dbg !231

bb5:                                              ; preds = %bb4
  br i1 %_7, label %bb6, label %bb8, !dbg !228

bb8:                                              ; preds = %bb5
  %5 = call zeroext i1 @"_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$i8$GT$3fmt17h5108e0376475bd4cE"(i8* align 1 %self, %"core::fmt::Formatter"* align 8 %f), !dbg !233
  %6 = zext i1 %5 to i8, !dbg !233
  store i8 %6, i8* %0, align 1, !dbg !233
  br label %bb9, !dbg !233

bb6:                                              ; preds = %bb5
  %7 = call zeroext i1 @"_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i8$GT$3fmt17h166a8ae1c63eec16E"(i8* align 1 %self, %"core::fmt::Formatter"* align 8 %f), !dbg !234
  %8 = zext i1 %7 to i8, !dbg !234
  store i8 %8, i8* %0, align 1, !dbg !234
  br label %bb7, !dbg !234

bb7:                                              ; preds = %bb6
  br label %bb10, !dbg !235

bb10:                                             ; preds = %bb9, %bb7
  br label %bb11, !dbg !230

bb9:                                              ; preds = %bb8
  br label %bb10, !dbg !235
}

; Function Attrs: nonlazybind uwtable
define align 8 %"core::fmt::builders::DebugList"* @_ZN4core3fmt8builders9DebugList7entries17hf305a3c344867e60E(%"core::fmt::builders::DebugList"* align 8 %self, i8* %entries.0, i8* %entries.1) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality !dbg !236 {
start:
  %0 = alloca { i8*, i32 }, align 8
  %entries.dbg.spill = alloca { i8*, i8* }, align 8
  %self.dbg.spill = alloca %"core::fmt::builders::DebugList"*, align 8
  %entry = alloca i8*, align 8
  %_7 = alloca i8*, align 8
  %iter = alloca { i8*, i8* }, align 8
  store %"core::fmt::builders::DebugList"* %self, %"core::fmt::builders::DebugList"** %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata %"core::fmt::builders::DebugList"** %self.dbg.spill, metadata !252, metadata !DIExpression()), !dbg !261
  %1 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %entries.dbg.spill, i32 0, i32 0
  store i8* %entries.0, i8** %1, align 8
  %2 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %entries.dbg.spill, i32 0, i32 1
  store i8* %entries.1, i8** %2, align 8
  call void @llvm.dbg.declare(metadata { i8*, i8* }* %entries.dbg.spill, metadata !253, metadata !DIExpression()), !dbg !262
  call void @llvm.dbg.declare(metadata { i8*, i8* }* %iter, metadata !254, metadata !DIExpression()), !dbg !263
  call void @llvm.dbg.declare(metadata i8** %entry, metadata !256, metadata !DIExpression()), !dbg !264
  %3 = call { i8*, i8* } @"_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17hd26588b6111d0594E"(i8* %entries.0, i8* %entries.1), !dbg !265
  %_4.0 = extractvalue { i8*, i8* } %3, 0, !dbg !265
  %_4.1 = extractvalue { i8*, i8* } %3, 1, !dbg !265
  br label %bb1, !dbg !265

bb1:                                              ; preds = %start
  %4 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %iter, i32 0, i32 0, !dbg !265
  store i8* %_4.0, i8** %4, align 8, !dbg !265
  %5 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %iter, i32 0, i32 1, !dbg !265
  store i8* %_4.1, i8** %5, align 8, !dbg !265
  br label %bb2, !dbg !266

bb2:                                              ; preds = %bb8, %bb1
  %6 = invoke align 1 i8* @"_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h71cf1614be9b7a2bE"({ i8*, i8* }* align 8 %iter)
          to label %bb3 unwind label %cleanup, !dbg !263

bb11:                                             ; preds = %bb10, %cleanup
  br label %bb12, !dbg !267

cleanup:                                          ; preds = %bb2
  %7 = landingpad { i8*, i32 }
          cleanup
  %8 = extractvalue { i8*, i32 } %7, 0
  %9 = extractvalue { i8*, i32 } %7, 1
  %10 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 0
  store i8* %8, i8** %10, align 8
  %11 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1
  store i32 %9, i32* %11, align 8
  br label %bb11

bb3:                                              ; preds = %bb2
  store i8* %6, i8** %_7, align 8, !dbg !263
  %12 = bitcast i8** %_7 to {}**, !dbg !263
  %13 = load {}*, {}** %12, align 8, !dbg !263
  %14 = icmp eq {}* %13, null, !dbg !263
  %_10 = select i1 %14, i64 0, i64 1, !dbg !263
  switch i64 %_10, label %bb5 [
    i64 0, label %bb6
    i64 1, label %bb4
  ], !dbg !263

bb5:                                              ; preds = %bb3
  unreachable, !dbg !263

bb6:                                              ; preds = %bb3
  br label %bb9, !dbg !267

bb4:                                              ; preds = %bb3
  %15 = load i8*, i8** %_7, align 8, !dbg !268, !nonnull !14, !align !149, !noundef !14
  store i8* %15, i8** %entry, align 8, !dbg !268
  %_14.0 = bitcast i8** %entry to {}*, !dbg !269
  %_12 = invoke align 8 %"core::fmt::builders::DebugList"* @_ZN4core3fmt8builders9DebugList5entry17hdb78004e117958ebE(%"core::fmt::builders::DebugList"* align 8 %self, {}* align 1 %_14.0, [3 x i64]* align 8 bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.0 to [3 x i64]*))
          to label %bb7 unwind label %cleanup1, !dbg !270

bb10:                                             ; preds = %cleanup1
  br label %bb11, !dbg !271

cleanup1:                                         ; preds = %bb4
  %16 = landingpad { i8*, i32 }
          cleanup
  %17 = extractvalue { i8*, i32 } %16, 0
  %18 = extractvalue { i8*, i32 } %16, 1
  %19 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 0
  store i8* %17, i8** %19, align 8
  %20 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1
  store i32 %18, i32* %20, align 8
  br label %bb10

bb7:                                              ; preds = %bb4
  br label %bb8, !dbg !271

bb8:                                              ; preds = %bb7
  br label %bb2, !dbg !266

bb12:                                             ; preds = %bb11
  %21 = bitcast { i8*, i32 }* %0 to i8**, !dbg !272
  %22 = load i8*, i8** %21, align 8, !dbg !272
  %23 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1, !dbg !272
  %24 = load i32, i32* %23, align 8, !dbg !272
  %25 = insertvalue { i8*, i32 } undef, i8* %22, 0, !dbg !272
  %26 = insertvalue { i8*, i32 } %25, i32 %24, 1, !dbg !272
  resume { i8*, i32 } %26, !dbg !272

bb9:                                              ; preds = %bb6
  ret %"core::fmt::builders::DebugList"* %self, !dbg !273
}

; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4core3ptr27drop_in_place$LT$$RF$i8$GT$17he47ffb12beaadcd1E"(i8** %_1) unnamed_addr #1 !dbg !274 {
start:
  %_1.dbg.spill = alloca i8**, align 8
  store i8** %_1, i8*** %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i8*** %_1.dbg.spill, metadata !280, metadata !DIExpression()), !dbg !281
  ret void, !dbg !281
}

; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4core3ptr49drop_in_place$LT$$RF$$u5b$i8$u3b$$u20$16$u5d$$GT$17h1b4d6bf73e0f167eE"([16 x i8]** %_1) unnamed_addr #1 !dbg !282 {
start:
  %_1.dbg.spill = alloca [16 x i8]**, align 8
  store [16 x i8]** %_1, [16 x i8]*** %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata [16 x i8]*** %_1.dbg.spill, metadata !287, metadata !DIExpression()), !dbg !290
  ret void, !dbg !290
}

; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN4core3ptr54drop_in_place$LT$$RF$$BP$mut$u20$core..ffi..c_void$GT$17hb669b5b1cf12a613E"(i8*** %_1) unnamed_addr #1 !dbg !291 {
start:
  %_1.dbg.spill = alloca i8***, align 8
  store i8*** %_1, i8**** %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i8**** %_1.dbg.spill, metadata !296, metadata !DIExpression()), !dbg !299
  ret void, !dbg !299
}

; Function Attrs: nonlazybind uwtable
define zeroext i1 @"_ZN4core5array69_$LT$impl$u20$core..fmt..Debug$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3fmt17h90ddc53a53953c41E"([16 x i8]* align 1 %self, %"core::fmt::Formatter"* align 8 %f) unnamed_addr #0 !dbg !300 {
start:
  %slice.dbg.spill = alloca { [0 x i8]*, i64 }, align 8
  %self.dbg.spill4 = alloca { [0 x i8]*, i64 }, align 8
  %self.dbg.spill3 = alloca [16 x i8]*, align 8
  %self.dbg.spill2 = alloca %"core::ops::range::RangeFull", align 1
  %index.dbg.spill1 = alloca %"core::ops::range::RangeFull", align 1
  %index.dbg.spill = alloca %"core::ops::range::RangeFull", align 1
  %f.dbg.spill = alloca %"core::fmt::Formatter"*, align 8
  %self.dbg.spill = alloca [16 x i8]*, align 8
  %_5 = alloca { [0 x i8]*, i64 }, align 8
  store [16 x i8]* %self, [16 x i8]** %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata [16 x i8]** %self.dbg.spill, metadata !307, metadata !DIExpression()), !dbg !309
  store %"core::fmt::Formatter"* %f, %"core::fmt::Formatter"** %f.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata %"core::fmt::Formatter"** %f.dbg.spill, metadata !308, metadata !DIExpression()), !dbg !310
  call void @llvm.dbg.declare(metadata %"core::ops::range::RangeFull"* %index.dbg.spill, metadata !311, metadata !DIExpression()), !dbg !336
  call void @llvm.dbg.declare(metadata %"core::ops::range::RangeFull"* %index.dbg.spill1, metadata !338, metadata !DIExpression()), !dbg !348
  call void @llvm.dbg.declare(metadata %"core::ops::range::RangeFull"* %self.dbg.spill2, metadata !350, metadata !DIExpression()), !dbg !358
  store [16 x i8]* %self, [16 x i8]** %self.dbg.spill3, align 8, !dbg !337
  call void @llvm.dbg.declare(metadata [16 x i8]** %self.dbg.spill3, metadata !333, metadata !DIExpression()), !dbg !336
  %_11.0 = bitcast [16 x i8]* %self to [0 x i8]*, !dbg !336
  %0 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %self.dbg.spill4, i32 0, i32 0, !dbg !336
  store [0 x i8]* %_11.0, [0 x i8]** %0, align 8, !dbg !336
  %1 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %self.dbg.spill4, i32 0, i32 1, !dbg !336
  store i64 16, i64* %1, align 8, !dbg !336
  call void @llvm.dbg.declare(metadata { [0 x i8]*, i64 }* %self.dbg.spill4, metadata !347, metadata !DIExpression()), !dbg !348
  %2 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %slice.dbg.spill, i32 0, i32 0, !dbg !348
  store [0 x i8]* %_11.0, [0 x i8]** %2, align 8, !dbg !348
  %3 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %slice.dbg.spill, i32 0, i32 1, !dbg !348
  store i64 16, i64* %3, align 8, !dbg !348
  call void @llvm.dbg.declare(metadata { [0 x i8]*, i64 }* %slice.dbg.spill, metadata !357, metadata !DIExpression()), !dbg !358
  %4 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %_5, i32 0, i32 0, !dbg !360
  store [0 x i8]* %_11.0, [0 x i8]** %4, align 8, !dbg !360
  %5 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %_5, i32 0, i32 1, !dbg !360
  store i64 16, i64* %5, align 8, !dbg !360
  %6 = call zeroext i1 @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h5311ceffab2d7116E"({ [0 x i8]*, i64 }* align 8 %_5, %"core::fmt::Formatter"* align 8 %f), !dbg !361
  br label %bb1, !dbg !361

bb1:                                              ; preds = %start
  ret i1 %6, !dbg !362
}

; Function Attrs: inlinehint nonlazybind uwtable
define { i8*, i8* } @"_ZN4core5slice4iter13Iter$LT$T$GT$3new17h34ce25feedef38b4E"([0 x i8]* align 1 %slice.0, i64 %slice.1) unnamed_addr #1 !dbg !363 {
start:
  %ptr.dbg.spill10 = alloca i8*, align 8
  %0 = alloca i8*, align 8
  %count.dbg.spill9 = alloca i64, align 8
  %self.dbg.spill8 = alloca i8*, align 8
  %count.dbg.spill7 = alloca i64, align 8
  %self.dbg.spill6 = alloca i8*, align 8
  %count.dbg.spill5 = alloca i64, align 8
  %self.dbg.spill4 = alloca i8*, align 8
  %count.dbg.spill = alloca i64, align 8
  %self.dbg.spill3 = alloca i8*, align 8
  %1 = alloca i8, align 1
  %other.dbg.spill = alloca i8*, align 8
  %data_address.dbg.spill = alloca {}*, align 8
  %2 = alloca {}*, align 8
  %self.dbg.spill2 = alloca i8*, align 8
  %self.dbg.spill1 = alloca i8*, align 8
  %ptr.dbg.spill = alloca i8*, align 8
  %self.dbg.spill = alloca { [0 x i8]*, i64 }, align 8
  %metadata.dbg.spill = alloca {}, align 1
  %slice.dbg.spill = alloca { [0 x i8]*, i64 }, align 8
  %_31 = alloca %"core::ptr::metadata::PtrComponents<u8>", align 8
  %_30 = alloca %"core::ptr::metadata::PtrRepr<u8>", align 8
  %_18 = alloca i8*, align 8
  %end = alloca i8*, align 8
  %3 = alloca { i8*, i8* }, align 8
  %4 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %slice.dbg.spill, i32 0, i32 0
  store [0 x i8]* %slice.0, [0 x i8]** %4, align 8
  %5 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %slice.dbg.spill, i32 0, i32 1
  store i64 %slice.1, i64* %5, align 8
  call void @llvm.dbg.declare(metadata { [0 x i8]*, i64 }* %slice.dbg.spill, metadata !366, metadata !DIExpression()), !dbg !371
  call void @llvm.dbg.declare(metadata i8** %end, metadata !369, metadata !DIExpression()), !dbg !372
  call void @llvm.dbg.declare(metadata {}* %metadata.dbg.spill, metadata !373, metadata !DIExpression()), !dbg !385
  %6 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %self.dbg.spill, i32 0, i32 0, !dbg !401
  store [0 x i8]* %slice.0, [0 x i8]** %6, align 8, !dbg !401
  %7 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %self.dbg.spill, i32 0, i32 1, !dbg !401
  store i64 %slice.1, i64* %7, align 8, !dbg !401
  call void @llvm.dbg.declare(metadata { [0 x i8]*, i64 }* %self.dbg.spill, metadata !402, metadata !DIExpression()), !dbg !408
  %ptr = bitcast [0 x i8]* %slice.0 to i8*, !dbg !408
  store i8* %ptr, i8** %ptr.dbg.spill, align 8, !dbg !408
  call void @llvm.dbg.declare(metadata i8** %ptr.dbg.spill, metadata !367, metadata !DIExpression()), !dbg !409
  store i8* %ptr, i8** %self.dbg.spill1, align 8, !dbg !400
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill1, metadata !398, metadata !DIExpression()), !dbg !410
  store i8* %ptr, i8** %self.dbg.spill2, align 8, !dbg !410
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill2, metadata !411, metadata !DIExpression()), !dbg !418
  %8 = bitcast {}** %2 to i64*, !dbg !420
  store i64 0, i64* %8, align 8, !dbg !420
  %data_address = load {}*, {}** %2, align 8, !dbg !420
  store {}* %data_address, {}** %data_address.dbg.spill, align 8, !dbg !420
  call void @llvm.dbg.declare(metadata {}** %data_address.dbg.spill, metadata !382, metadata !DIExpression()), !dbg !385
  br label %bb5, !dbg !420

bb5:                                              ; preds = %start
  %9 = bitcast %"core::ptr::metadata::PtrComponents<u8>"* %_31 to {}**, !dbg !385
  store {}* %data_address, {}** %9, align 8, !dbg !385
  %10 = bitcast %"core::ptr::metadata::PtrRepr<u8>"* %_30 to %"core::ptr::metadata::PtrComponents<u8>"*, !dbg !385
  %11 = bitcast %"core::ptr::metadata::PtrComponents<u8>"* %10 to i8*, !dbg !385
  %12 = bitcast %"core::ptr::metadata::PtrComponents<u8>"* %_31 to i8*, !dbg !385
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %11, i8* align 8 %12, i64 8, i1 false), !dbg !385
  %13 = bitcast %"core::ptr::metadata::PtrRepr<u8>"* %_30 to i8**, !dbg !385
  %other = load i8*, i8** %13, align 8, !dbg !385
  store i8* %other, i8** %other.dbg.spill, align 8, !dbg !385
  call void @llvm.dbg.declare(metadata i8** %other.dbg.spill, metadata !417, metadata !DIExpression()), !dbg !418
  %14 = icmp eq i8* %ptr, %other, !dbg !418
  %15 = zext i1 %14 to i8, !dbg !418
  store i8 %15, i8* %1, align 1, !dbg !418
  %16 = load i8, i8* %1, align 1, !dbg !418, !range !232, !noundef !14
  %_6 = trunc i8 %16 to i1, !dbg !418
  br label %bb6, !dbg !418

bb6:                                              ; preds = %bb5
  %_5 = xor i1 %_6, true, !dbg !429
  call void @llvm.assume(i1 %_5), !dbg !430
  br label %bb1, !dbg !430

bb1:                                              ; preds = %bb6
  br i1 false, label %bb2, label %bb3, !dbg !431

bb2:                                              ; preds = %bb1
  store i8* %ptr, i8** %self.dbg.spill6, align 8, !dbg !432
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill6, metadata !433, metadata !DIExpression()), !dbg !440
  store i64 %slice.1, i64* %count.dbg.spill7, align 8, !dbg !441
  call void @llvm.dbg.declare(metadata i64* %count.dbg.spill7, metadata !439, metadata !DIExpression()), !dbg !440
  store i8* %ptr, i8** %self.dbg.spill8, align 8, !dbg !440
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill8, metadata !442, metadata !DIExpression()), !dbg !450
  store i64 %slice.1, i64* %count.dbg.spill9, align 8, !dbg !440
  call void @llvm.dbg.declare(metadata i64* %count.dbg.spill9, metadata !449, metadata !DIExpression()), !dbg !450
  %17 = getelementptr i8, i8* %ptr, i64 %slice.1, !dbg !450
  store i8* %17, i8** %0, align 8, !dbg !450
  %_10 = load i8*, i8** %0, align 8, !dbg !450
  br label %bb7, !dbg !450

bb3:                                              ; preds = %bb1
  store i8* %ptr, i8** %self.dbg.spill3, align 8, !dbg !452
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill3, metadata !453, metadata !DIExpression()), !dbg !460
  store i64 %slice.1, i64* %count.dbg.spill, align 8, !dbg !461
  call void @llvm.dbg.declare(metadata i64* %count.dbg.spill, metadata !459, metadata !DIExpression()), !dbg !460
  store i8* %ptr, i8** %self.dbg.spill4, align 8, !dbg !460
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill4, metadata !462, metadata !DIExpression()), !dbg !469
  store i64 %slice.1, i64* %count.dbg.spill5, align 8, !dbg !460
  call void @llvm.dbg.declare(metadata i64* %count.dbg.spill5, metadata !468, metadata !DIExpression()), !dbg !469
  %18 = getelementptr inbounds i8, i8* %ptr, i64 %slice.1, !dbg !469
  store i8* %18, i8** %end, align 8, !dbg !469
  br label %bb8, !dbg !469

bb8:                                              ; preds = %bb3
  br label %bb4, !dbg !471

bb4:                                              ; preds = %bb7, %bb8
  store i8* %ptr, i8** %ptr.dbg.spill10, align 8, !dbg !472
  call void @llvm.dbg.declare(metadata i8** %ptr.dbg.spill10, metadata !473, metadata !DIExpression()), !dbg !481
  store i8* %ptr, i8** %_18, align 8, !dbg !481
  %_21 = load i8*, i8** %end, align 8, !dbg !483
  %19 = bitcast { i8*, i8* }* %3 to i8**, !dbg !484
  %20 = load i8*, i8** %_18, align 8, !dbg !484, !nonnull !14, !noundef !14
  store i8* %20, i8** %19, align 8, !dbg !484
  %21 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %3, i32 0, i32 1, !dbg !484
  store i8* %_21, i8** %21, align 8, !dbg !484
  %22 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %3, i32 0, i32 0, !dbg !485
  %23 = load i8*, i8** %22, align 8, !dbg !485, !nonnull !14, !noundef !14
  %24 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %3, i32 0, i32 1, !dbg !485
  %25 = load i8*, i8** %24, align 8, !dbg !485
  %26 = insertvalue { i8*, i8* } undef, i8* %23, 0, !dbg !485
  %27 = insertvalue { i8*, i8* } %26, i8* %25, 1, !dbg !485
  ret { i8*, i8* } %27, !dbg !485

bb7:                                              ; preds = %bb2
  store i8* %_10, i8** %end, align 8, !dbg !432
  br label %bb4, !dbg !471
}

; Function Attrs: nonlazybind uwtable
define zeroext i1 @"_ZN50_$LT$$BP$mut$u20$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h8c2be5a5f4e50655E"(i8** align 8 %self, %"core::fmt::Formatter"* align 8 %f) unnamed_addr #0 !dbg !486 {
start:
  %f.dbg.spill = alloca %"core::fmt::Formatter"*, align 8
  %self.dbg.spill = alloca i8**, align 8
  store i8** %self, i8*** %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i8*** %self.dbg.spill, metadata !491, metadata !DIExpression()), !dbg !495
  store %"core::fmt::Formatter"* %f, %"core::fmt::Formatter"** %f.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata %"core::fmt::Formatter"** %f.dbg.spill, metadata !492, metadata !DIExpression()), !dbg !496
  %0 = call zeroext i1 @"_ZN52_$LT$$BP$mut$u20$T$u20$as$u20$core..fmt..Pointer$GT$3fmt17h3086c8efd06e963aE"(i8** align 8 %self, %"core::fmt::Formatter"* align 8 %f), !dbg !497
  br label %bb1, !dbg !497

bb1:                                              ; preds = %start
  ret i1 %0, !dbg !498
}

; Function Attrs: nonlazybind uwtable
define zeroext i1 @"_ZN52_$LT$$BP$mut$u20$T$u20$as$u20$core..fmt..Pointer$GT$3fmt17h3086c8efd06e963aE"(i8** align 8 %self, %"core::fmt::Formatter"* align 8 %f) unnamed_addr #0 !dbg !499 {
start:
  %f.dbg.spill = alloca %"core::fmt::Formatter"*, align 8
  %self.dbg.spill = alloca i8**, align 8
  %_5 = alloca i8*, align 8
  store i8** %self, i8*** %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i8*** %self.dbg.spill, metadata !502, metadata !DIExpression()), !dbg !504
  store %"core::fmt::Formatter"* %f, %"core::fmt::Formatter"** %f.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata %"core::fmt::Formatter"** %f.dbg.spill, metadata !503, metadata !DIExpression()), !dbg !505
  %_6 = load i8*, i8** %self, align 8, !dbg !506
  store i8* %_6, i8** %_5, align 8, !dbg !506
  %0 = call zeroext i1 @"_ZN54_$LT$$BP$const$u20$T$u20$as$u20$core..fmt..Pointer$GT$3fmt17h5ce58e97e8216fadE"(i8** align 8 %_5, %"core::fmt::Formatter"* align 8 %f), !dbg !507
  br label %bb1, !dbg !507

bb1:                                              ; preds = %start
  ret i1 %0, !dbg !508
}

; Function Attrs: nonlazybind uwtable
define zeroext i1 @"_ZN54_$LT$$BP$const$u20$T$u20$as$u20$core..fmt..Pointer$GT$3fmt17h5ce58e97e8216fadE"(i8** align 8 %self, %"core::fmt::Formatter"* align 8 %f) unnamed_addr #0 !dbg !509 {
start:
  %0 = alloca i64, align 8
  %self.dbg.spill2 = alloca {}*, align 8
  %f.dbg.spill = alloca %"core::fmt::Formatter"*, align 8
  %self.dbg.spill = alloca i8**, align 8
  store i8** %self, i8*** %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata i8*** %self.dbg.spill, metadata !516, metadata !DIExpression()), !dbg !518
  store %"core::fmt::Formatter"* %f, %"core::fmt::Formatter"** %f.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata %"core::fmt::Formatter"** %f.dbg.spill, metadata !517, metadata !DIExpression()), !dbg !519
  %_5 = load i8*, i8** %self, align 8, !dbg !520
  %self1 = bitcast i8* %_5 to {}*, !dbg !521
  store {}* %self1, {}** %self.dbg.spill2, align 8, !dbg !521
  call void @llvm.dbg.declare(metadata {}** %self.dbg.spill2, metadata !522, metadata !DIExpression()), !dbg !528
  %1 = bitcast i64* %0 to {}**, !dbg !528
  store {}* %self1, {}** %1, align 8, !dbg !528
  %_3 = load i64, i64* %0, align 8, !dbg !528
  br label %bb2, !dbg !528

bb2:                                              ; preds = %start
  %2 = call zeroext i1 @_ZN4core3fmt17pointer_fmt_inner17h9a09ed045cb3c6abE(i64 %_3, %"core::fmt::Formatter"* align 8 %f), !dbg !529
  br label %bb1, !dbg !529

bb1:                                              ; preds = %bb2
  ret i1 %2, !dbg !530
}

; Function Attrs: inlinehint nonlazybind uwtable
define { i8*, i8* } @"_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17hd26588b6111d0594E"(i8* %self.0, i8* %self.1) unnamed_addr #1 !dbg !531 {
start:
  %self.dbg.spill = alloca { i8*, i8* }, align 8
  %0 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %self.dbg.spill, i32 0, i32 0
  store i8* %self.0, i8** %0, align 8
  %1 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %self.dbg.spill, i32 0, i32 1
  store i8* %self.1, i8** %1, align 8
  call void @llvm.dbg.declare(metadata { i8*, i8* }* %self.dbg.spill, metadata !540, metadata !DIExpression()), !dbg !542
  %2 = insertvalue { i8*, i8* } undef, i8* %self.0, 0, !dbg !543
  %3 = insertvalue { i8*, i8* } %2, i8* %self.1, 1, !dbg !543
  ret { i8*, i8* } %3, !dbg !543
}

; Function Attrs: inlinehint nonlazybind uwtable
define align 1 i8* @"_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h71cf1614be9b7a2bE"({ i8*, i8* }* align 8 %self) unnamed_addr #1 !dbg !544 {
start:
  %self.dbg.spill23 = alloca i8*, align 8
  %0 = alloca i8*, align 8
  %self.dbg.spill21 = alloca i8*, align 8
  %ptr.dbg.spill = alloca i8*, align 8
  %1 = alloca i8*, align 8
  %self.dbg.spill20 = alloca i8*, align 8
  %self.dbg.spill19 = alloca i8*, align 8
  %old.dbg.spill = alloca i8*, align 8
  %self.dbg.spill17 = alloca i8*, align 8
  %self.dbg.spill15 = alloca { i8*, i8* }*, align 8
  %self.dbg.spill14 = alloca i8*, align 8
  %2 = alloca i8, align 1
  %other.dbg.spill12 = alloca i8*, align 8
  %data_address.dbg.spill10 = alloca {}*, align 8
  %3 = alloca {}*, align 8
  %self.dbg.spill8 = alloca i8*, align 8
  %self.dbg.spill7 = alloca i8*, align 8
  %4 = alloca i8, align 1
  %other.dbg.spill = alloca i8*, align 8
  %data_address.dbg.spill = alloca {}*, align 8
  %5 = alloca {}*, align 8
  %self.dbg.spill5 = alloca i8*, align 8
  %self.dbg.spill4 = alloca i8*, align 8
  %self.dbg.spill3 = alloca i8*, align 8
  %metadata.dbg.spill1 = alloca {}, align 1
  %metadata.dbg.spill = alloca {}, align 1
  %self.dbg.spill = alloca { i8*, i8* }*, align 8
  %_62 = alloca i8*, align 8
  %_47 = alloca %"core::ptr::metadata::PtrComponents<u8>", align 8
  %_46 = alloca %"core::ptr::metadata::PtrRepr<u8>", align 8
  %_30 = alloca %"core::ptr::metadata::PtrComponents<u8>", align 8
  %_29 = alloca %"core::ptr::metadata::PtrRepr<u8>", align 8
  %_19 = alloca i8*, align 8
  %6 = alloca i8*, align 8
  store { i8*, i8* }* %self, { i8*, i8* }** %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata { i8*, i8* }** %self.dbg.spill, metadata !562, metadata !DIExpression()), !dbg !563
  call void @llvm.dbg.declare(metadata {}* %metadata.dbg.spill, metadata !564, metadata !DIExpression()), !dbg !573
  call void @llvm.dbg.declare(metadata {}* %metadata.dbg.spill1, metadata !589, metadata !DIExpression()), !dbg !594
  %7 = bitcast { i8*, i8* }* %self to i8**, !dbg !588
  %self2 = load i8*, i8** %7, align 8, !dbg !588, !nonnull !14, !noundef !14
  store i8* %self2, i8** %self.dbg.spill3, align 8, !dbg !588
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill3, metadata !603, metadata !DIExpression()), !dbg !609
  store i8* %self2, i8** %self.dbg.spill4, align 8, !dbg !609
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill4, metadata !586, metadata !DIExpression()), !dbg !610
  store i8* %self2, i8** %self.dbg.spill5, align 8, !dbg !610
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill5, metadata !611, metadata !DIExpression()), !dbg !618
  %8 = bitcast {}** %5 to i64*, !dbg !620
  store i64 0, i64* %8, align 8, !dbg !620
  %data_address = load {}*, {}** %5, align 8, !dbg !620
  store {}* %data_address, {}** %data_address.dbg.spill, align 8, !dbg !620
  call void @llvm.dbg.declare(metadata {}** %data_address.dbg.spill, metadata !572, metadata !DIExpression()), !dbg !573
  br label %bb9, !dbg !620

bb9:                                              ; preds = %start
  %9 = bitcast %"core::ptr::metadata::PtrComponents<u8>"* %_30 to {}**, !dbg !573
  store {}* %data_address, {}** %9, align 8, !dbg !573
  %10 = bitcast %"core::ptr::metadata::PtrRepr<u8>"* %_29 to %"core::ptr::metadata::PtrComponents<u8>"*, !dbg !573
  %11 = bitcast %"core::ptr::metadata::PtrComponents<u8>"* %10 to i8*, !dbg !573
  %12 = bitcast %"core::ptr::metadata::PtrComponents<u8>"* %_30 to i8*, !dbg !573
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %11, i8* align 8 %12, i64 8, i1 false), !dbg !573
  %13 = bitcast %"core::ptr::metadata::PtrRepr<u8>"* %_29 to i8**, !dbg !573
  %other = load i8*, i8** %13, align 8, !dbg !573
  store i8* %other, i8** %other.dbg.spill, align 8, !dbg !573
  call void @llvm.dbg.declare(metadata i8** %other.dbg.spill, metadata !617, metadata !DIExpression()), !dbg !618
  %14 = icmp eq i8* %self2, %other, !dbg !618
  %15 = zext i1 %14 to i8, !dbg !618
  store i8 %15, i8* %4, align 1, !dbg !618
  %16 = load i8, i8* %4, align 1, !dbg !618, !range !232, !noundef !14
  %_4 = trunc i8 %16 to i1, !dbg !618
  br label %bb10, !dbg !618

bb10:                                             ; preds = %bb9
  %_3 = xor i1 %_4, true, !dbg !628
  call void @llvm.assume(i1 %_3), !dbg !629
  br label %bb1, !dbg !629

bb1:                                              ; preds = %bb10
  br i1 false, label %bb4, label %bb2, !dbg !630

bb4:                                              ; preds = %bb1
  br label %bb5, !dbg !631

bb2:                                              ; preds = %bb1
  %17 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %self, i32 0, i32 1, !dbg !602
  %self6 = load i8*, i8** %17, align 8, !dbg !602
  store i8* %self6, i8** %self.dbg.spill7, align 8, !dbg !602
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill7, metadata !600, metadata !DIExpression()), !dbg !632
  store i8* %self6, i8** %self.dbg.spill8, align 8, !dbg !632
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill8, metadata !633, metadata !DIExpression()), !dbg !638
  %18 = bitcast {}** %3 to i64*, !dbg !640
  store i64 0, i64* %18, align 8, !dbg !640
  %data_address9 = load {}*, {}** %3, align 8, !dbg !640
  store {}* %data_address9, {}** %data_address.dbg.spill10, align 8, !dbg !640
  call void @llvm.dbg.declare(metadata {}** %data_address.dbg.spill10, metadata !593, metadata !DIExpression()), !dbg !594
  br label %bb11, !dbg !640

bb11:                                             ; preds = %bb2
  %19 = bitcast %"core::ptr::metadata::PtrComponents<u8>"* %_47 to {}**, !dbg !594
  store {}* %data_address9, {}** %19, align 8, !dbg !594
  %20 = bitcast %"core::ptr::metadata::PtrRepr<u8>"* %_46 to %"core::ptr::metadata::PtrComponents<u8>"*, !dbg !594
  %21 = bitcast %"core::ptr::metadata::PtrComponents<u8>"* %20 to i8*, !dbg !594
  %22 = bitcast %"core::ptr::metadata::PtrComponents<u8>"* %_47 to i8*, !dbg !594
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %21, i8* align 8 %22, i64 8, i1 false), !dbg !594
  %23 = bitcast %"core::ptr::metadata::PtrRepr<u8>"* %_46 to i8**, !dbg !594
  %other11 = load i8*, i8** %23, align 8, !dbg !594
  store i8* %other11, i8** %other.dbg.spill12, align 8, !dbg !594
  call void @llvm.dbg.declare(metadata i8** %other.dbg.spill12, metadata !637, metadata !DIExpression()), !dbg !638
  %24 = icmp eq i8* %self6, %other11, !dbg !638
  %25 = zext i1 %24 to i8, !dbg !638
  store i8 %25, i8* %2, align 1, !dbg !638
  %26 = load i8, i8* %2, align 1, !dbg !638, !range !232, !noundef !14
  %_10 = trunc i8 %26 to i1, !dbg !638
  br label %bb12, !dbg !638

bb12:                                             ; preds = %bb11
  %_9 = xor i1 %_10, true, !dbg !646
  call void @llvm.assume(i1 %_9), !dbg !647
  br label %bb3, !dbg !647

bb3:                                              ; preds = %bb12
  br label %bb5, !dbg !631

bb5:                                              ; preds = %bb3, %bb4
  %27 = bitcast { i8*, i8* }* %self to i8**, !dbg !648
  %self13 = load i8*, i8** %27, align 8, !dbg !648, !nonnull !14, !noundef !14
  store i8* %self13, i8** %self.dbg.spill14, align 8, !dbg !648
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill14, metadata !649, metadata !DIExpression()), !dbg !653
  %28 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %self, i32 0, i32 1, !dbg !648
  %_16 = load i8*, i8** %28, align 8, !dbg !648
  %_12 = icmp eq i8* %self13, %_16, !dbg !648
  br i1 %_12, label %bb6, label %bb7, !dbg !648

bb7:                                              ; preds = %bb5
  store { i8*, i8* }* %self, { i8*, i8* }** %self.dbg.spill15, align 8, !dbg !655
  call void @llvm.dbg.declare(metadata { i8*, i8* }** %self.dbg.spill15, metadata !656, metadata !DIExpression()), !dbg !664
  br i1 false, label %bb13, label %bb14, !dbg !664

bb6:                                              ; preds = %bb5
  %29 = bitcast i8** %6 to {}**, !dbg !666
  store {}* null, {}** %29, align 8, !dbg !666
  br label %bb8, !dbg !667

bb8:                                              ; preds = %bb15, %bb6
  %30 = load i8*, i8** %6, align 8, !dbg !668, !align !149
  ret i8* %30, !dbg !668

bb13:                                             ; preds = %bb7
  %31 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %self, i32 0, i32 1, !dbg !664
  %_56 = load i8*, i8** %31, align 8, !dbg !664
  store i8* %_56, i8** %self.dbg.spill21, align 8, !dbg !664
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill21, metadata !669, metadata !DIExpression()), !dbg !674
  %32 = getelementptr i8, i8* %_56, i64 -1, !dbg !674
  store i8* %32, i8** %0, align 8, !dbg !674
  %_54 = load i8*, i8** %0, align 8, !dbg !674
  br label %bb16, !dbg !674

bb14:                                             ; preds = %bb7
  %33 = bitcast { i8*, i8* }* %self to i8**, !dbg !664
  %self16 = load i8*, i8** %33, align 8, !dbg !664, !nonnull !14, !noundef !14
  store i8* %self16, i8** %self.dbg.spill17, align 8, !dbg !664
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill17, metadata !676, metadata !DIExpression()), !dbg !680
  store i8* %self16, i8** %old.dbg.spill, align 8, !dbg !680
  call void @llvm.dbg.declare(metadata i8** %old.dbg.spill, metadata !662, metadata !DIExpression()), !dbg !682
  %34 = bitcast { i8*, i8* }* %self to i8**, !dbg !682
  %self18 = load i8*, i8** %34, align 8, !dbg !682, !nonnull !14, !noundef !14
  store i8* %self18, i8** %self.dbg.spill19, align 8, !dbg !682
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill19, metadata !683, metadata !DIExpression()), !dbg !687
  store i8* %self18, i8** %self.dbg.spill20, align 8, !dbg !687
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill20, metadata !689, metadata !DIExpression()), !dbg !696
  %35 = getelementptr inbounds i8, i8* %self18, i64 1, !dbg !696
  store i8* %35, i8** %1, align 8, !dbg !696
  %_73 = load i8*, i8** %1, align 8, !dbg !696
  br label %bb17, !dbg !696

bb17:                                             ; preds = %bb14
  store i8* %_73, i8** %ptr.dbg.spill, align 8, !dbg !696
  call void @llvm.dbg.declare(metadata i8** %ptr.dbg.spill, metadata !697, metadata !DIExpression()), !dbg !701
  store i8* %_73, i8** %_62, align 8, !dbg !701
  %36 = bitcast { i8*, i8* }* %self to i8**, !dbg !682
  %37 = load i8*, i8** %_62, align 8, !dbg !682, !nonnull !14, !noundef !14
  store i8* %37, i8** %36, align 8, !dbg !682
  store i8* %self16, i8** %_19, align 8, !dbg !682
  br label %bb15, !dbg !664

bb15:                                             ; preds = %bb16, %bb17
  %_18 = load i8*, i8** %_19, align 8, !dbg !655
  store i8* %_18, i8** %6, align 8, !dbg !703
  br label %bb8, !dbg !667

bb16:                                             ; preds = %bb13
  %38 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %self, i32 0, i32 1, !dbg !664
  store i8* %_54, i8** %38, align 8, !dbg !664
  %39 = bitcast { i8*, i8* }* %self to i8**, !dbg !664
  %self22 = load i8*, i8** %39, align 8, !dbg !664, !nonnull !14, !noundef !14
  store i8* %self22, i8** %self.dbg.spill23, align 8, !dbg !664
  call void @llvm.dbg.declare(metadata i8** %self.dbg.spill23, metadata !704, metadata !DIExpression()), !dbg !708
  store i8* %self22, i8** %_19, align 8, !dbg !664
  br label %bb15, !dbg !664
}

; Function Attrs: nonlazybind uwtable
define void @_ZN3sys5Hello8sayHello17hc58b692bab97984bE(%Hello* align 8 %self) unnamed_addr #0 !dbg !710 {
start:
  %self.dbg.spill = alloca %Hello*, align 8
  store %Hello* %self, %Hello** %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata %Hello** %self.dbg.spill, metadata !721, metadata !DIExpression()), !dbg !722
  call void @"\01_ZN5Hello8sayHelloEv"(%Hello* %self), !dbg !723
  br label %bb1, !dbg !723

bb1:                                              ; preds = %start
  ret void, !dbg !724
}

; Function Attrs: nonlazybind uwtable
define void @_ZN3sys5Hello3new17h3c7bea36bb2a3dc0E(%Hello* sret(%Hello) %0) unnamed_addr #0 !dbg !725 {
start:
  %slot.i = alloca %"core::mem::manually_drop::ManuallyDrop<Hello>", align 8
  %self.dbg.spill.i = alloca %"core::mem::maybe_uninit::MaybeUninit<Hello>"*, align 8
  %_5 = alloca %"core::mem::maybe_uninit::MaybeUninit<Hello>", align 8
  %__bindgen_tmp = alloca %"core::mem::maybe_uninit::MaybeUninit<Hello>", align 8
  call void @llvm.dbg.declare(metadata %"core::mem::maybe_uninit::MaybeUninit<Hello>"* %__bindgen_tmp, metadata !729, metadata !DIExpression()), !dbg !743
  %1 = bitcast %"core::mem::maybe_uninit::MaybeUninit<Hello>"* %__bindgen_tmp to {}*, !dbg !744
  br label %bb1, !dbg !750

bb1:                                              ; preds = %start
  store %"core::mem::maybe_uninit::MaybeUninit<Hello>"* %__bindgen_tmp, %"core::mem::maybe_uninit::MaybeUninit<Hello>"** %self.dbg.spill.i, align 8
  call void @llvm.dbg.declare(metadata %"core::mem::maybe_uninit::MaybeUninit<Hello>"** %self.dbg.spill.i, metadata !751, metadata !DIExpression()), !dbg !758
  %2 = bitcast %"core::mem::maybe_uninit::MaybeUninit<Hello>"* %__bindgen_tmp to %Hello*, !dbg !760
  br label %bb2, !dbg !761

bb2:                                              ; preds = %bb1
  call void @"\01_ZN5HelloC1Ev"(%Hello* %2), !dbg !762
  br label %bb3, !dbg !762

bb3:                                              ; preds = %bb2
  %3 = bitcast %"core::mem::maybe_uninit::MaybeUninit<Hello>"* %_5 to i8*, !dbg !763
  %4 = bitcast %"core::mem::maybe_uninit::MaybeUninit<Hello>"* %__bindgen_tmp to i8*, !dbg !763
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %3, i8* align 8 %4, i64 24, i1 false), !dbg !763
  call void @llvm.dbg.declare(metadata %"core::mem::maybe_uninit::MaybeUninit<Hello>"* %_5, metadata !764, metadata !DIExpression()), !dbg !769
  call void @llvm.dbg.declare(metadata %"core::mem::manually_drop::ManuallyDrop<Hello>"* %slot.i, metadata !771, metadata !DIExpression()), !dbg !778
  %5 = bitcast %"core::mem::maybe_uninit::MaybeUninit<Hello>"* %_5 to %"core::mem::manually_drop::ManuallyDrop<Hello>"*, !dbg !780
  %6 = bitcast %"core::mem::manually_drop::ManuallyDrop<Hello>"* %slot.i to i8*, !dbg !780
  %7 = bitcast %"core::mem::manually_drop::ManuallyDrop<Hello>"* %5 to i8*, !dbg !780
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %6, i8* align 8 %7, i64 24, i1 false), !dbg !780
  %8 = bitcast %"core::mem::manually_drop::ManuallyDrop<Hello>"* %slot.i to %Hello*, !dbg !778
  %9 = bitcast %Hello* %0 to i8*, !dbg !778
  %10 = bitcast %Hello* %8 to i8*, !dbg !778
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %9, i8* align 8 %10, i64 24, i1 false), !dbg !778
  br label %bb4, !dbg !763

bb4:                                              ; preds = %bb3
  ret void, !dbg !781
}

; Function Attrs: nonlazybind uwtable
define void @_ZN3sys5Hello8destruct17h5ed162002c7eb0c2E(%Hello* align 8 %self) unnamed_addr #0 !dbg !782 {
start:
  %self.dbg.spill = alloca %Hello*, align 8
  store %Hello* %self, %Hello** %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata %Hello** %self.dbg.spill, metadata !784, metadata !DIExpression()), !dbg !785
  call void @"\01_ZN5HelloD1Ev"(%Hello* %self), !dbg !786
  br label %bb1, !dbg !786

bb1:                                              ; preds = %start
  ret void, !dbg !787
}

; Function Attrs: nonlazybind uwtable
define zeroext i1 @"_ZN47_$LT$sys..Hello$u20$as$u20$core..fmt..Debug$GT$3fmt17h1a8253c24512449bE"(%Hello* align 8 %self, %"core::fmt::Formatter"* align 8 %f) unnamed_addr #0 !dbg !788 {
start:
  %f.dbg.spill = alloca %"core::fmt::Formatter"*, align 8
  %self.dbg.spill = alloca %Hello*, align 8
  %_17 = alloca i8**, align 8
  %_11 = alloca [16 x i8]*, align 8
  store %Hello* %self, %Hello** %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata %Hello** %self.dbg.spill, metadata !794, metadata !DIExpression()), !dbg !796
  store %"core::fmt::Formatter"* %f, %"core::fmt::Formatter"** %f.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata %"core::fmt::Formatter"** %f.dbg.spill, metadata !795, metadata !DIExpression()), !dbg !796
  %0 = bitcast %Hello* %self to [16 x i8]*, !dbg !797
  store [16 x i8]* %0, [16 x i8]** %_11, align 8, !dbg !797
  %_8.0 = bitcast [16 x i8]** %_11 to {}*, !dbg !797
  %1 = getelementptr inbounds %Hello, %Hello* %self, i32 0, i32 1, !dbg !798
  store i8** %1, i8*** %_17, align 8, !dbg !798
  %_14.0 = bitcast i8*** %_17 to {}*, !dbg !798
  %2 = call zeroext i1 @_ZN4core3fmt9Formatter26debug_struct_field2_finish17he19226b6f261a1eeE(%"core::fmt::Formatter"* align 8 %f, [0 x i8]* align 1 bitcast (<{ [5 x i8] }>* @alloc43 to [0 x i8]*), i64 5, [0 x i8]* align 1 bitcast (<{ [4 x i8] }>* @alloc44 to [0 x i8]*), i64 4, {}* align 1 %_8.0, [3 x i64]* align 8 bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.1 to [3 x i64]*), [0 x i8]* align 1 bitcast (<{ [3 x i8] }>* @alloc48 to [0 x i8]*), i64 3, {}* align 1 %_14.0, [3 x i64]* align 8 bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.2 to [3 x i64]*)), !dbg !796
  br label %bb1, !dbg !796

bb1:                                              ; preds = %start
  ret i1 %2, !dbg !799
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #2

; Function Attrs: nonlazybind uwtable
declare void @_ZN4core3fmt9Formatter10debug_list17hdc92f4eec9879f86E(%"core::fmt::builders::DebugList"* sret(%"core::fmt::builders::DebugList"), %"core::fmt::Formatter"* align 8) unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare zeroext i1 @_ZN4core3fmt8builders9DebugList6finish17h6ba6f640472a04e8E(%"core::fmt::builders::DebugList"* align 8) unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare zeroext i1 @_ZN4core3fmt9Formatter15debug_lower_hex17h0be447fd1ce6c68cE(%"core::fmt::Formatter"* align 8) unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i8$GT$3fmt17hb40b1edf2ea57794E"(i8* align 1, %"core::fmt::Formatter"* align 8) unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare zeroext i1 @_ZN4core3fmt9Formatter15debug_upper_hex17he97a574bdc69fc4aE(%"core::fmt::Formatter"* align 8) unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i8$GT$3fmt17h166a8ae1c63eec16E"(i8* align 1, %"core::fmt::Formatter"* align 8) unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$i8$GT$3fmt17h5108e0376475bd4cE"(i8* align 1, %"core::fmt::Formatter"* align 8) unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare align 8 %"core::fmt::builders::DebugList"* @_ZN4core3fmt8builders9DebugList5entry17hdb78004e117958ebE(%"core::fmt::builders::DebugList"* align 8, {}* align 1, [3 x i64]* align 8) unnamed_addr #0

; Function Attrs: argmemonly nofree nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #3

; Function Attrs: inaccessiblememonly nofree nosync nounwind willreturn
declare void @llvm.assume(i1 noundef) #4

; Function Attrs: nonlazybind uwtable
declare zeroext i1 @_ZN4core3fmt17pointer_fmt_inner17h9a09ed045cb3c6abE(i64, %"core::fmt::Formatter"* align 8) unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare void @"\01_ZN5Hello8sayHelloEv"(%Hello*) unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare void @"\01_ZN5HelloC1Ev"(%Hello*) unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare void @"\01_ZN5HelloD1Ev"(%Hello*) unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare zeroext i1 @_ZN4core3fmt9Formatter26debug_struct_field2_finish17he19226b6f261a1eeE(%"core::fmt::Formatter"* align 8, [0 x i8]* align 1, i64, [0 x i8]* align 1, i64, {}* align 1, [3 x i64]* align 8, [0 x i8]* align 1, i64, {}* align 1, [3 x i64]* align 8) unnamed_addr #0

attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { inlinehint nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #2 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #3 = { argmemonly nofree nounwind willreturn }
attributes #4 = { inaccessiblememonly nofree nosync nounwind willreturn }

!llvm.module.flags = !{!44, !45, !46, !47}
!llvm.dbg.cu = !{!48}

!0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
!1 = distinct !DIGlobalVariable(name: "<&i8 as core::fmt::Debug>::{vtable}", scope: null, file: !2, type: !3, isLocal: true, isDefinition: true)
!2 = !DIFile(filename: "<unknown>", directory: "")
!3 = !DICompositeType(tag: DW_TAG_structure_type, name: "<&i8 as core::fmt::Debug>::{vtable_type}", file: !2, size: 256, align: 64, flags: DIFlagArtificial, elements: !4, vtableHolder: !12, templateParams: !14, identifier: "fe09b79eceaa265313aba7160dc723cb")
!4 = !{!5, !8, !10, !11}
!5 = !DIDerivedType(tag: DW_TAG_member, name: "drop_in_place", scope: !3, file: !2, baseType: !6, size: 64, align: 64)
!6 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const ()", baseType: !7, size: 64, align: 64, dwarfAddressSpace: 0)
!7 = !DIBasicType(name: "()", encoding: DW_ATE_unsigned)
!8 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !3, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!9 = !DIBasicType(name: "usize", size: 64, encoding: DW_ATE_unsigned)
!10 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !3, file: !2, baseType: !9, size: 64, align: 64, offset: 128)
!11 = !DIDerivedType(tag: DW_TAG_member, name: "__method3", scope: !3, file: !2, baseType: !6, size: 64, align: 64, offset: 192)
!12 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&i8", baseType: !13, size: 64, align: 64, dwarfAddressSpace: 0)
!13 = !DIBasicType(name: "i8", size: 8, encoding: DW_ATE_signed)
!14 = !{}
!15 = !DIGlobalVariableExpression(var: !16, expr: !DIExpression())
!16 = distinct !DIGlobalVariable(name: "<&[i8; 16] as core::fmt::Debug>::{vtable}", scope: null, file: !2, type: !17, isLocal: true, isDefinition: true)
!17 = !DICompositeType(tag: DW_TAG_structure_type, name: "<&[i8; 16] as core::fmt::Debug>::{vtable_type}", file: !2, size: 256, align: 64, flags: DIFlagArtificial, elements: !18, vtableHolder: !23, templateParams: !14, identifier: "b2f72894d4700236c9275c05006bdac6")
!18 = !{!19, !20, !21, !22}
!19 = !DIDerivedType(tag: DW_TAG_member, name: "drop_in_place", scope: !17, file: !2, baseType: !6, size: 64, align: 64)
!20 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !17, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!21 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !17, file: !2, baseType: !9, size: 64, align: 64, offset: 128)
!22 = !DIDerivedType(tag: DW_TAG_member, name: "__method3", scope: !17, file: !2, baseType: !6, size: 64, align: 64, offset: 192)
!23 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[i8; 16]", baseType: !24, size: 64, align: 64, dwarfAddressSpace: 0)
!24 = !DICompositeType(tag: DW_TAG_array_type, baseType: !13, size: 128, align: 8, elements: !25)
!25 = !{!26}
!26 = !DISubrange(count: 16, lowerBound: 0)
!27 = !DIGlobalVariableExpression(var: !28, expr: !DIExpression())
!28 = distinct !DIGlobalVariable(name: "<&*mut core::ffi::c_void as core::fmt::Debug>::{vtable}", scope: null, file: !2, type: !29, isLocal: true, isDefinition: true)
!29 = !DICompositeType(tag: DW_TAG_structure_type, name: "<&*mut core::ffi::c_void as core::fmt::Debug>::{vtable_type}", file: !2, size: 256, align: 64, flags: DIFlagArtificial, elements: !30, vtableHolder: !35, templateParams: !14, identifier: "fbd7b64445a5c522c0c95ae931f01bed")
!30 = !{!31, !32, !33, !34}
!31 = !DIDerivedType(tag: DW_TAG_member, name: "drop_in_place", scope: !29, file: !2, baseType: !6, size: 64, align: 64)
!32 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !29, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!33 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !29, file: !2, baseType: !9, size: 64, align: 64, offset: 128)
!34 = !DIDerivedType(tag: DW_TAG_member, name: "__method3", scope: !29, file: !2, baseType: !6, size: 64, align: 64, offset: 192)
!35 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&*mut core::ffi::c_void", baseType: !36, size: 64, align: 64, dwarfAddressSpace: 0)
!36 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut core::ffi::c_void", baseType: !37, size: 64, align: 64, dwarfAddressSpace: 0)
!37 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "c_void", scope: !38, file: !2, baseType: !40, size: 8, align: 8, flags: DIFlagEnumClass, elements: !41)
!38 = !DINamespace(name: "ffi", scope: !39)
!39 = !DINamespace(name: "core", scope: null)
!40 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!41 = !{!42, !43}
!42 = !DIEnumerator(name: "__variant1", value: 0, isUnsigned: true)
!43 = !DIEnumerator(name: "__variant2", value: 1, isUnsigned: true)
!44 = !{i32 7, !"PIC Level", i32 2}
!45 = !{i32 2, !"RtLibUseGOT", i32 1}
!46 = !{i32 2, !"Dwarf Version", i32 4}
!47 = !{i32 2, !"Debug Info Version", i32 3}
!48 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !49, producer: "clang LLVM (rustc version 1.64.0-nightly (f9cba6374 2022-07-31))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !50, globals: !60)
!49 = !DIFile(filename: "sys/src/lib.rs/@/2ph5xhx6fjxfiqym", directory: "/home/ubuntu/Workspace/RustCLA/ABIChecker/tests/wstest")
!50 = !{!37, !51}
!51 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Alignment", scope: !52, file: !2, baseType: !40, size: 8, align: 8, flags: DIFlagEnumClass, elements: !55)
!52 = !DINamespace(name: "v1", scope: !53)
!53 = !DINamespace(name: "rt", scope: !54)
!54 = !DINamespace(name: "fmt", scope: !39)
!55 = !{!56, !57, !58, !59}
!56 = !DIEnumerator(name: "Left", value: 0, isUnsigned: true)
!57 = !DIEnumerator(name: "Right", value: 1, isUnsigned: true)
!58 = !DIEnumerator(name: "Center", value: 2, isUnsigned: true)
!59 = !DIEnumerator(name: "Unknown", value: 3, isUnsigned: true)
!60 = !{!0, !15, !27}
!61 = distinct !DISubprogram(name: "fmt<*mut core::ffi::c_void>", linkageName: "_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h2eeb052d2a18da27E", scope: !63, file: !62, line: 2361, type: !64, scopeLine: 2361, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !124, retainedNodes: !121)
!62 = !DIFile(filename: "/rustc/f9cba63746d0fff816250b2ba7b706b5d4dcf000/library/core/src/fmt/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "18dcc19de419985ae78d2bd8ed07e5dc")
!63 = !DINamespace(name: "{impl#59}", scope: !54)
!64 = !DISubroutineType(types: !65)
!65 = !{!66, !84, !85}
!66 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<(), core::fmt::Error>", scope: !67, file: !2, size: 8, align: 8, elements: !68, templateParams: !14, identifier: "7d2d8f8fa7cd1adfcd0790efb9a41e68")
!67 = !DINamespace(name: "result", scope: !39)
!68 = !{!69}
!69 = !DICompositeType(tag: DW_TAG_variant_part, scope: !66, file: !2, size: 8, align: 8, elements: !70, templateParams: !14, identifier: "da4053b05ceaafbb2a5d0c162ca3dc6", discriminator: !83)
!70 = !{!71, !79}
!71 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !69, file: !2, baseType: !72, size: 8, align: 8, extraData: i64 0)
!72 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !66, file: !2, size: 8, align: 8, elements: !73, templateParams: !75, identifier: "8661b9e657e06668dc2c12fa2eb06bc5")
!73 = !{!74}
!74 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !72, file: !2, baseType: !7, align: 8, offset: 8)
!75 = !{!76, !77}
!76 = !DITemplateTypeParameter(name: "T", type: !7)
!77 = !DITemplateTypeParameter(name: "E", type: !78)
!78 = !DICompositeType(tag: DW_TAG_structure_type, name: "Error", scope: !54, file: !2, align: 8, elements: !14, identifier: "fc98057bd5303a3616baccc8ca680c9")
!79 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !69, file: !2, baseType: !80, size: 8, align: 8, extraData: i64 1)
!80 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !66, file: !2, size: 8, align: 8, elements: !81, templateParams: !75, identifier: "7c5b987816b912a975f1f8ff7d771c4c")
!81 = !{!82}
!82 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !80, file: !2, baseType: !78, align: 8, offset: 8)
!83 = !DIDerivedType(tag: DW_TAG_member, scope: !66, file: !2, baseType: !40, size: 8, align: 8, flags: DIFlagArtificial)
!84 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&&*mut core::ffi::c_void", baseType: !35, size: 64, align: 64, dwarfAddressSpace: 0)
!85 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::fmt::Formatter", baseType: !86, size: 64, align: 64, dwarfAddressSpace: 0)
!86 = !DICompositeType(tag: DW_TAG_structure_type, name: "Formatter", scope: !54, file: !2, size: 512, align: 64, elements: !87, templateParams: !14, identifier: "1fafe706266f1ce911c5c21ffa099199")
!87 = !{!88, !90, !92, !93, !109, !110}
!88 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !86, file: !2, baseType: !89, size: 32, align: 32, offset: 384)
!89 = !DIBasicType(name: "u32", size: 32, encoding: DW_ATE_unsigned)
!90 = !DIDerivedType(tag: DW_TAG_member, name: "fill", scope: !86, file: !2, baseType: !91, size: 32, align: 32, offset: 416)
!91 = !DIBasicType(name: "char", size: 32, encoding: DW_ATE_UTF)
!92 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !86, file: !2, baseType: !51, size: 8, align: 8, offset: 448)
!93 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !86, file: !2, baseType: !94, size: 128, align: 64)
!94 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<usize>", scope: !95, file: !2, size: 128, align: 64, elements: !96, templateParams: !14, identifier: "5738eb22aef83125d64176aca9b7b21c")
!95 = !DINamespace(name: "option", scope: !39)
!96 = !{!97}
!97 = !DICompositeType(tag: DW_TAG_variant_part, scope: !94, file: !2, size: 128, align: 64, elements: !98, templateParams: !14, identifier: "b6c90093930ab1adb411d543c3709fb8", discriminator: !107)
!98 = !{!99, !103}
!99 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !97, file: !2, baseType: !100, size: 128, align: 64, extraData: i64 0)
!100 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !94, file: !2, size: 128, align: 64, elements: !14, templateParams: !101, identifier: "a8804933eea6abc51bcee581a3a61566")
!101 = !{!102}
!102 = !DITemplateTypeParameter(name: "T", type: !9)
!103 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !97, file: !2, baseType: !104, size: 128, align: 64, extraData: i64 1)
!104 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !94, file: !2, size: 128, align: 64, elements: !105, templateParams: !101, identifier: "fd013aa7409f7a44b4ed55c961a937ef")
!105 = !{!106}
!106 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !104, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!107 = !DIDerivedType(tag: DW_TAG_member, scope: !94, file: !2, baseType: !108, size: 64, align: 64, flags: DIFlagArtificial)
!108 = !DIBasicType(name: "u64", size: 64, encoding: DW_ATE_unsigned)
!109 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !86, file: !2, baseType: !94, size: 128, align: 64, offset: 128)
!110 = !DIDerivedType(tag: DW_TAG_member, name: "buf", scope: !86, file: !2, baseType: !111, size: 128, align: 64, offset: 256)
!111 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut dyn core::fmt::Write", file: !2, size: 128, align: 64, elements: !112, templateParams: !14, identifier: "3e37c7a363d2ba763ef179a883dd555b")
!112 = !{!113, !116}
!113 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !111, file: !2, baseType: !114, size: 64, align: 64)
!114 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !115, size: 64, align: 64, dwarfAddressSpace: 0)
!115 = !DICompositeType(tag: DW_TAG_structure_type, name: "dyn core::fmt::Write", file: !2, align: 8, elements: !14, identifier: "95cfa81398e3251d3addeb69d75cd2f4")
!116 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !111, file: !2, baseType: !117, size: 64, align: 64, offset: 64)
!117 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 3]", baseType: !118, size: 64, align: 64, dwarfAddressSpace: 0)
!118 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 192, align: 64, elements: !119)
!119 = !{!120}
!120 = !DISubrange(count: 3, lowerBound: 0)
!121 = !{!122, !123}
!122 = !DILocalVariable(name: "self", arg: 1, scope: !61, file: !62, line: 2361, type: !84)
!123 = !DILocalVariable(name: "f", arg: 2, scope: !61, file: !62, line: 2361, type: !85)
!124 = !{!125}
!125 = !DITemplateTypeParameter(name: "T", type: !36)
!126 = !DILocation(line: 2361, column: 20, scope: !61)
!127 = !DILocation(line: 2361, column: 27, scope: !61)
!128 = !DILocation(line: 2361, column: 71, scope: !61)
!129 = !{i64 8}
!130 = !DILocation(line: 2361, column: 62, scope: !61)
!131 = !DILocation(line: 2361, column: 84, scope: !61)
!132 = distinct !DISubprogram(name: "fmt<[i8]>", linkageName: "_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h5311ceffab2d7116E", scope: !63, file: !62, line: 2361, type: !133, scopeLine: 2361, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !141)
!133 = !DISubroutineType(types: !134)
!134 = !{!66, !135, !85}
!135 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&&[i8]", baseType: !136, size: 64, align: 64, dwarfAddressSpace: 0)
!136 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[i8]", file: !2, size: 128, align: 64, elements: !137, templateParams: !14, identifier: "cc2ad320ed202ac3c13bb57e7e44d101")
!137 = !{!138, !140}
!138 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !136, file: !2, baseType: !139, size: 64, align: 64)
!139 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !13, size: 64, align: 64, dwarfAddressSpace: 0)
!140 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !136, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!141 = !{!142, !143}
!142 = !DILocalVariable(name: "self", arg: 1, scope: !132, file: !62, line: 2361, type: !135)
!143 = !DILocalVariable(name: "f", arg: 2, scope: !132, file: !62, line: 2361, type: !85)
!144 = !{!145}
!145 = !DITemplateTypeParameter(name: "T", type: !13)
!146 = !DILocation(line: 2361, column: 20, scope: !132)
!147 = !DILocation(line: 2361, column: 27, scope: !132)
!148 = !DILocation(line: 2361, column: 71, scope: !132)
!149 = !{i64 1}
!150 = !DILocation(line: 2361, column: 62, scope: !132)
!151 = !DILocation(line: 2361, column: 84, scope: !132)
!152 = distinct !DISubprogram(name: "fmt<i8>", linkageName: "_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h9d33819da977a53aE", scope: !63, file: !62, line: 2361, type: !153, scopeLine: 2361, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !156)
!153 = !DISubroutineType(types: !154)
!154 = !{!66, !155, !85}
!155 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&&i8", baseType: !12, size: 64, align: 64, dwarfAddressSpace: 0)
!156 = !{!157, !158}
!157 = !DILocalVariable(name: "self", arg: 1, scope: !152, file: !62, line: 2361, type: !155)
!158 = !DILocalVariable(name: "f", arg: 2, scope: !152, file: !62, line: 2361, type: !85)
!159 = !DILocation(line: 2361, column: 20, scope: !152)
!160 = !DILocation(line: 2361, column: 27, scope: !152)
!161 = !DILocation(line: 2361, column: 71, scope: !152)
!162 = !DILocation(line: 2361, column: 62, scope: !152)
!163 = !DILocation(line: 2361, column: 84, scope: !152)
!164 = distinct !DISubprogram(name: "fmt<[i8; 16]>", linkageName: "_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hba762ccd28fafa81E", scope: !63, file: !62, line: 2361, type: !165, scopeLine: 2361, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !171, retainedNodes: !168)
!165 = !DISubroutineType(types: !166)
!166 = !{!66, !167, !85}
!167 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&&[i8; 16]", baseType: !23, size: 64, align: 64, dwarfAddressSpace: 0)
!168 = !{!169, !170}
!169 = !DILocalVariable(name: "self", arg: 1, scope: !164, file: !62, line: 2361, type: !167)
!170 = !DILocalVariable(name: "f", arg: 2, scope: !164, file: !62, line: 2361, type: !85)
!171 = !{!172}
!172 = !DITemplateTypeParameter(name: "T", type: !24)
!173 = !DILocation(line: 2361, column: 20, scope: !164)
!174 = !DILocation(line: 2361, column: 27, scope: !164)
!175 = !DILocation(line: 2361, column: 71, scope: !164)
!176 = !DILocation(line: 2361, column: 62, scope: !164)
!177 = !DILocation(line: 2361, column: 84, scope: !164)
!178 = distinct !DISubprogram(name: "fmt<i8>", linkageName: "_ZN48_$LT$$u5b$T$u5d$$u20$as$u20$core..fmt..Debug$GT$3fmt17hbe5f57de97c67132E", scope: !179, file: !62, line: 2586, type: !180, scopeLine: 2586, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !182)
!179 = !DINamespace(name: "{impl#26}", scope: !54)
!180 = !DISubroutineType(types: !181)
!181 = !{!66, !136, !85}
!182 = !{!183, !184}
!183 = !DILocalVariable(name: "self", arg: 1, scope: !178, file: !62, line: 2586, type: !136)
!184 = !DILocalVariable(name: "f", arg: 2, scope: !178, file: !62, line: 2586, type: !85)
!185 = !DILocation(line: 2586, column: 12, scope: !178)
!186 = !DILocation(line: 2586, column: 19, scope: !178)
!187 = !DILocation(line: 2587, column: 9, scope: !178)
!188 = !DILocation(line: 2587, column: 32, scope: !178)
!189 = !DILocalVariable(name: "self", scope: !190, file: !62, line: 2587, type: !136, align: 8)
!190 = !DILexicalBlockFile(scope: !191, file: !62, discriminator: 0)
!191 = distinct !DISubprogram(name: "iter<i8>", linkageName: "_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$4iter17heb57ca83f7d75c30E", scope: !193, file: !192, line: 734, type: !195, scopeLine: 734, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !213)
!192 = !DIFile(filename: "/rustc/f9cba63746d0fff816250b2ba7b706b5d4dcf000/library/core/src/slice/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "7e4712ab5341970456c5be6b0829b712")
!193 = !DINamespace(name: "{impl#0}", scope: !194)
!194 = !DINamespace(name: "slice", scope: !39)
!195 = !DISubroutineType(types: !196)
!196 = !{!197, !136}
!197 = !DICompositeType(tag: DW_TAG_structure_type, name: "Iter<i8>", scope: !198, file: !2, size: 128, align: 64, elements: !199, templateParams: !144, identifier: "607fdfb9e3bd1b3fe5f70d3e162def1")
!198 = !DINamespace(name: "iter", scope: !194)
!199 = !{!200, !207, !208}
!200 = !DIDerivedType(tag: DW_TAG_member, name: "ptr", scope: !197, file: !2, baseType: !201, size: 64, align: 64)
!201 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<i8>", scope: !202, file: !2, size: 64, align: 64, elements: !204, templateParams: !144, identifier: "cf7e44a501fb1983d2b3109bbb9039c")
!202 = !DINamespace(name: "non_null", scope: !203)
!203 = !DINamespace(name: "ptr", scope: !39)
!204 = !{!205}
!205 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !201, file: !2, baseType: !206, size: 64, align: 64)
!206 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const i8", baseType: !13, size: 64, align: 64, dwarfAddressSpace: 0)
!207 = !DIDerivedType(tag: DW_TAG_member, name: "end", scope: !197, file: !2, baseType: !206, size: 64, align: 64, offset: 64)
!208 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !197, file: !2, baseType: !209, align: 8)
!209 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<&i8>", scope: !210, file: !2, align: 8, elements: !14, templateParams: !211, identifier: "adb409230bca9b2acbeff7b1df5913d3")
!210 = !DINamespace(name: "marker", scope: !39)
!211 = !{!212}
!212 = !DITemplateTypeParameter(name: "T", type: !12)
!213 = !{!189}
!214 = !DILocation(line: 2587, column: 32, scope: !190, inlinedAt: !188)
!215 = !DILocation(line: 2588, column: 6, scope: !178)
!216 = distinct !DISubprogram(name: "fmt", linkageName: "_ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i8$GT$3fmt17h32a029fc41c52f79E", scope: !218, file: !217, line: 185, type: !220, scopeLine: 185, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !14, retainedNodes: !222)
!217 = !DIFile(filename: "/rustc/f9cba63746d0fff816250b2ba7b706b5d4dcf000/library/core/src/fmt/num.rs", directory: "", checksumkind: CSK_MD5, checksum: "6661e46781fcd4293e0f5caf32f3d8ca")
!218 = !DINamespace(name: "{impl#78}", scope: !219)
!219 = !DINamespace(name: "num", scope: !54)
!220 = !DISubroutineType(types: !221)
!221 = !{!66, !12, !85}
!222 = !{!223, !224}
!223 = !DILocalVariable(name: "self", arg: 1, scope: !216, file: !217, line: 185, type: !12)
!224 = !DILocalVariable(name: "f", arg: 2, scope: !216, file: !217, line: 185, type: !85)
!225 = !DILocation(line: 185, column: 20, scope: !216)
!226 = !DILocation(line: 185, column: 27, scope: !216)
!227 = !DILocation(line: 186, column: 20, scope: !216)
!228 = !DILocation(line: 188, column: 27, scope: !216)
!229 = !DILocation(line: 187, column: 21, scope: !216)
!230 = !DILocation(line: 186, column: 17, scope: !216)
!231 = !DILocation(line: 193, column: 14, scope: !216)
!232 = !{i8 0, i8 2}
!233 = !DILocation(line: 191, column: 21, scope: !216)
!234 = !DILocation(line: 189, column: 21, scope: !216)
!235 = !DILocation(line: 188, column: 24, scope: !216)
!236 = distinct !DISubprogram(name: "entries<&i8, core::slice::iter::Iter<i8>>", linkageName: "_ZN4core3fmt8builders9DebugList7entries17hf305a3c344867e60E", scope: !238, file: !237, line: 637, type: !248, scopeLine: 637, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !258, retainedNodes: !251)
!237 = !DIFile(filename: "/rustc/f9cba63746d0fff816250b2ba7b706b5d4dcf000/library/core/src/fmt/builders.rs", directory: "", checksumkind: CSK_MD5, checksum: "e84d8f928a38ea86b324aa7d6aa80df8")
!238 = !DICompositeType(tag: DW_TAG_structure_type, name: "DebugList", scope: !239, file: !2, size: 128, align: 64, elements: !240, templateParams: !14, identifier: "bf3d540e7d7cbcaf67b9a3926afbfc1f")
!239 = !DINamespace(name: "builders", scope: !54)
!240 = !{!241}
!241 = !DIDerivedType(tag: DW_TAG_member, name: "inner", scope: !238, file: !2, baseType: !242, size: 128, align: 64)
!242 = !DICompositeType(tag: DW_TAG_structure_type, name: "DebugInner", scope: !239, file: !2, size: 128, align: 64, elements: !243, templateParams: !14, identifier: "4032c44244787b6244349163798d999b")
!243 = !{!244, !245, !246}
!244 = !DIDerivedType(tag: DW_TAG_member, name: "fmt", scope: !242, file: !2, baseType: !85, size: 64, align: 64)
!245 = !DIDerivedType(tag: DW_TAG_member, name: "result", scope: !242, file: !2, baseType: !66, size: 8, align: 8, offset: 64)
!246 = !DIDerivedType(tag: DW_TAG_member, name: "has_fields", scope: !242, file: !2, baseType: !247, size: 8, align: 8, offset: 72)
!247 = !DIBasicType(name: "bool", size: 8, encoding: DW_ATE_boolean)
!248 = !DISubroutineType(types: !249)
!249 = !{!250, !250, !197}
!250 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::fmt::builders::DebugList", baseType: !238, size: 64, align: 64, dwarfAddressSpace: 0)
!251 = !{!252, !253, !254, !256}
!252 = !DILocalVariable(name: "self", arg: 1, scope: !236, file: !237, line: 637, type: !250)
!253 = !DILocalVariable(name: "entries", arg: 2, scope: !236, file: !237, line: 637, type: !197)
!254 = !DILocalVariable(name: "iter", scope: !255, file: !237, line: 642, type: !197, align: 8)
!255 = distinct !DILexicalBlock(scope: !236, file: !237, line: 642, column: 9)
!256 = !DILocalVariable(name: "entry", scope: !257, file: !237, line: 642, type: !12, align: 8)
!257 = distinct !DILexicalBlock(scope: !255, file: !237, line: 642, column: 30)
!258 = !{!259, !260}
!259 = !DITemplateTypeParameter(name: "D", type: !12)
!260 = !DITemplateTypeParameter(name: "I", type: !197)
!261 = !DILocation(line: 637, column: 26, scope: !236)
!262 = !DILocation(line: 637, column: 37, scope: !236)
!263 = !DILocation(line: 642, column: 22, scope: !255)
!264 = !DILocation(line: 642, column: 13, scope: !257)
!265 = !DILocation(line: 642, column: 22, scope: !236)
!266 = !DILocation(line: 642, column: 9, scope: !255)
!267 = !DILocation(line: 642, column: 9, scope: !236)
!268 = !DILocation(line: 642, column: 13, scope: !255)
!269 = !DILocation(line: 643, column: 24, scope: !257)
!270 = !DILocation(line: 643, column: 13, scope: !257)
!271 = !DILocation(line: 644, column: 9, scope: !255)
!272 = !DILocation(line: 637, column: 5, scope: !236)
!273 = !DILocation(line: 646, column: 6, scope: !236)
!274 = distinct !DISubprogram(name: "drop_in_place<&i8>", linkageName: "_ZN4core3ptr27drop_in_place$LT$$RF$i8$GT$17he47ffb12beaadcd1E", scope: !203, file: !275, line: 487, type: !276, scopeLine: 487, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !211, retainedNodes: !279)
!275 = !DIFile(filename: "/rustc/f9cba63746d0fff816250b2ba7b706b5d4dcf000/library/core/src/ptr/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "13c32d970b0b4dd38131a1908223a155")
!276 = !DISubroutineType(types: !277)
!277 = !{null, !278}
!278 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut &i8", baseType: !12, size: 64, align: 64, dwarfAddressSpace: 0)
!279 = !{!280}
!280 = !DILocalVariable(arg: 1, scope: !274, file: !275, line: 487, type: !278)
!281 = !DILocation(line: 487, column: 1, scope: !274)
!282 = distinct !DISubprogram(name: "drop_in_place<&[i8; 16]>", linkageName: "_ZN4core3ptr49drop_in_place$LT$$RF$$u5b$i8$u3b$$u20$16$u5d$$GT$17h1b4d6bf73e0f167eE", scope: !203, file: !275, line: 487, type: !283, scopeLine: 487, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !288, retainedNodes: !286)
!283 = !DISubroutineType(types: !284)
!284 = !{null, !285}
!285 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut &[i8; 16]", baseType: !23, size: 64, align: 64, dwarfAddressSpace: 0)
!286 = !{!287}
!287 = !DILocalVariable(arg: 1, scope: !282, file: !275, line: 487, type: !285)
!288 = !{!289}
!289 = !DITemplateTypeParameter(name: "T", type: !23)
!290 = !DILocation(line: 487, column: 1, scope: !282)
!291 = distinct !DISubprogram(name: "drop_in_place<&*mut core::ffi::c_void>", linkageName: "_ZN4core3ptr54drop_in_place$LT$$RF$$BP$mut$u20$core..ffi..c_void$GT$17hb669b5b1cf12a613E", scope: !203, file: !275, line: 487, type: !292, scopeLine: 487, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !297, retainedNodes: !295)
!292 = !DISubroutineType(types: !293)
!293 = !{null, !294}
!294 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut &*mut core::ffi::c_void", baseType: !35, size: 64, align: 64, dwarfAddressSpace: 0)
!295 = !{!296}
!296 = !DILocalVariable(arg: 1, scope: !291, file: !275, line: 487, type: !294)
!297 = !{!298}
!298 = !DITemplateTypeParameter(name: "T", type: !35)
!299 = !DILocation(line: 487, column: 1, scope: !291)
!300 = distinct !DISubprogram(name: "fmt<i8, 16>", linkageName: "_ZN4core5array69_$LT$impl$u20$core..fmt..Debug$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3fmt17h90ddc53a53953c41E", scope: !302, file: !301, line: 251, type: !304, scopeLine: 251, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !306)
!301 = !DIFile(filename: "/rustc/f9cba63746d0fff816250b2ba7b706b5d4dcf000/library/core/src/array/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "e237aff39bff30ac55984ee183267020")
!302 = !DINamespace(name: "{impl#12}", scope: !303)
!303 = !DINamespace(name: "array", scope: !39)
!304 = !DISubroutineType(types: !305)
!305 = !{!66, !23, !85}
!306 = !{!307, !308}
!307 = !DILocalVariable(name: "self", arg: 1, scope: !300, file: !301, line: 251, type: !23)
!308 = !DILocalVariable(name: "f", arg: 2, scope: !300, file: !301, line: 251, type: !85)
!309 = !DILocation(line: 251, column: 12, scope: !300)
!310 = !DILocation(line: 251, column: 19, scope: !300)
!311 = !DILocalVariable(name: "index", scope: !312, file: !301, line: 252, type: !316, align: 1)
!312 = distinct !DISubprogram(name: "index<i8, core::ops::range::RangeFull, 16>", linkageName: "_ZN4core5array85_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$5index17h6fc7d2f733963a6fE", scope: !313, file: !301, line: 285, type: !314, scopeLine: 285, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !334, retainedNodes: !332)
!313 = !DINamespace(name: "{impl#15}", scope: !303)
!314 = !DISubroutineType(types: !315)
!315 = !{!136, !23, !316, !319}
!316 = !DICompositeType(tag: DW_TAG_structure_type, name: "RangeFull", scope: !317, file: !2, align: 8, elements: !14, identifier: "ea4ae9024d3deb506fb80ea738434b3c")
!317 = !DINamespace(name: "range", scope: !318)
!318 = !DINamespace(name: "ops", scope: !39)
!319 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::panic::location::Location", baseType: !320, size: 64, align: 64, dwarfAddressSpace: 0)
!320 = !DICompositeType(tag: DW_TAG_structure_type, name: "Location", scope: !321, file: !2, size: 192, align: 64, elements: !323, templateParams: !14, identifier: "7956c41cd54ddb276a0e7c46e033f72f")
!321 = !DINamespace(name: "location", scope: !322)
!322 = !DINamespace(name: "panic", scope: !39)
!323 = !{!324, !330, !331}
!324 = !DIDerivedType(tag: DW_TAG_member, name: "file", scope: !320, file: !2, baseType: !325, size: 128, align: 64)
!325 = !DICompositeType(tag: DW_TAG_structure_type, name: "&str", file: !2, size: 128, align: 64, elements: !326, templateParams: !14, identifier: "c603ebb2af364502ef89131a86c6ad9b")
!326 = !{!327, !329}
!327 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !325, file: !2, baseType: !328, size: 64, align: 64)
!328 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !40, size: 64, align: 64, dwarfAddressSpace: 0)
!329 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !325, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!330 = !DIDerivedType(tag: DW_TAG_member, name: "line", scope: !320, file: !2, baseType: !89, size: 32, align: 32, offset: 128)
!331 = !DIDerivedType(tag: DW_TAG_member, name: "col", scope: !320, file: !2, baseType: !89, size: 32, align: 32, offset: 160)
!332 = !{!333, !311}
!333 = !DILocalVariable(name: "self", scope: !312, file: !301, line: 252, type: !23, align: 8)
!334 = !{!145, !335}
!335 = !DITemplateTypeParameter(name: "I", type: !316)
!336 = !DILocation(line: 252, column: 27, scope: !312, inlinedAt: !337)
!337 = !DILocation(line: 252, column: 27, scope: !300)
!338 = !DILocalVariable(name: "index", scope: !339, file: !301, line: 252, type: !316, align: 1)
!339 = !DILexicalBlockFile(scope: !340, file: !301, discriminator: 0)
!340 = distinct !DISubprogram(name: "index<i8, core::ops::range::RangeFull>", linkageName: "_ZN4core5slice5index74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17hfd381bdb9c0204dfE", scope: !342, file: !341, line: 17, type: !344, scopeLine: 17, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !334, retainedNodes: !346)
!341 = !DIFile(filename: "/rustc/f9cba63746d0fff816250b2ba7b706b5d4dcf000/library/core/src/slice/index.rs", directory: "", checksumkind: CSK_MD5, checksum: "22a04328eca3d78fd2bf5357f9150928")
!342 = !DINamespace(name: "{impl#0}", scope: !343)
!343 = !DINamespace(name: "index", scope: !194)
!344 = !DISubroutineType(types: !345)
!345 = !{!136, !136, !316, !319}
!346 = !{!347, !338}
!347 = !DILocalVariable(name: "self", scope: !339, file: !301, line: 252, type: !136, align: 8)
!348 = !DILocation(line: 252, column: 27, scope: !339, inlinedAt: !349)
!349 = !DILocation(line: 286, column: 9, scope: !312, inlinedAt: !337)
!350 = !DILocalVariable(name: "self", scope: !351, file: !301, line: 252, type: !316, align: 1)
!351 = !DILexicalBlockFile(scope: !352, file: !301, discriminator: 0)
!352 = distinct !DISubprogram(name: "index<i8>", linkageName: "_ZN97_$LT$core..ops..range..RangeFull$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17h581103a5a6f04f99E", scope: !353, file: !341, line: 432, type: !354, scopeLine: 432, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !356)
!353 = !DINamespace(name: "{impl#6}", scope: !343)
!354 = !DISubroutineType(types: !355)
!355 = !{!136, !316, !136, !319}
!356 = !{!350, !357}
!357 = !DILocalVariable(name: "slice", scope: !351, file: !301, line: 252, type: !136, align: 8)
!358 = !DILocation(line: 252, column: 27, scope: !351, inlinedAt: !359)
!359 = !DILocation(line: 18, column: 9, scope: !340, inlinedAt: !349)
!360 = !DILocation(line: 252, column: 26, scope: !300)
!361 = !DILocation(line: 252, column: 9, scope: !300)
!362 = !DILocation(line: 253, column: 6, scope: !300)
!363 = distinct !DISubprogram(name: "new<i8>", linkageName: "_ZN4core5slice4iter13Iter$LT$T$GT$3new17h34ce25feedef38b4E", scope: !197, file: !364, line: 88, type: !195, scopeLine: 88, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !365)
!364 = !DIFile(filename: "/rustc/f9cba63746d0fff816250b2ba7b706b5d4dcf000/library/core/src/slice/iter.rs", directory: "", checksumkind: CSK_MD5, checksum: "aaf9347bbb50ba703d4a9c31ccf40fa6")
!365 = !{!366, !367, !369}
!366 = !DILocalVariable(name: "slice", arg: 1, scope: !363, file: !364, line: 88, type: !136)
!367 = !DILocalVariable(name: "ptr", scope: !368, file: !364, line: 89, type: !206, align: 8)
!368 = distinct !DILexicalBlock(scope: !363, file: !364, line: 89, column: 9)
!369 = !DILocalVariable(name: "end", scope: !370, file: !364, line: 94, type: !206, align: 8)
!370 = distinct !DILexicalBlock(scope: !368, file: !364, line: 94, column: 13)
!371 = !DILocation(line: 88, column: 23, scope: !363)
!372 = !DILocation(line: 94, column: 17, scope: !370)
!373 = !DILocalVariable(name: "metadata", scope: !374, file: !364, line: 92, type: !7, align: 1)
!374 = !DILexicalBlockFile(scope: !375, file: !364, discriminator: 0)
!375 = distinct !DISubprogram(name: "from_raw_parts<u8>", linkageName: "_ZN4core3ptr8metadata14from_raw_parts17h7d57f7befca6c5abE", scope: !377, file: !376, line: 110, type: !378, scopeLine: 110, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !383, retainedNodes: !381)
!376 = !DIFile(filename: "/rustc/f9cba63746d0fff816250b2ba7b706b5d4dcf000/library/core/src/ptr/metadata.rs", directory: "", checksumkind: CSK_MD5, checksum: "59565ed3c34dee3e8f8928c29f8f7ce4")
!377 = !DINamespace(name: "metadata", scope: !203)
!378 = !DISubroutineType(types: !379)
!379 = !{!380, !6, !7}
!380 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u8", baseType: !40, size: 64, align: 64, dwarfAddressSpace: 0)
!381 = !{!382, !373}
!382 = !DILocalVariable(name: "data_address", scope: !374, file: !364, line: 92, type: !6, align: 8)
!383 = !{!384}
!384 = !DITemplateTypeParameter(name: "T", type: !40)
!385 = !DILocation(line: 92, column: 21, scope: !374, inlinedAt: !386)
!386 = !DILocation(line: 513, column: 5, scope: !387, inlinedAt: !390)
!387 = distinct !DISubprogram(name: "null<u8>", linkageName: "_ZN4core3ptr4null17habef1b51f6ef211dE", scope: !203, file: !275, line: 512, type: !388, scopeLine: 512, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !383, retainedNodes: !14)
!388 = !DISubroutineType(types: !389)
!389 = !{!380}
!390 = !DILocation(line: 39, column: 43, scope: !391, inlinedAt: !400)
!391 = distinct !DISubprogram(name: "is_null<i8>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7is_null17h6244b6be9c3b154eE", scope: !393, file: !392, line: 36, type: !395, scopeLine: 36, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !397)
!392 = !DIFile(filename: "/rustc/f9cba63746d0fff816250b2ba7b706b5d4dcf000/library/core/src/ptr/const_ptr.rs", directory: "", checksumkind: CSK_MD5, checksum: "6b6635c3709e36ccb9099e622886f9c9")
!393 = !DINamespace(name: "{impl#0}", scope: !394)
!394 = !DINamespace(name: "const_ptr", scope: !203)
!395 = !DISubroutineType(types: !396)
!396 = !{!247, !206}
!397 = !{!398}
!398 = !DILocalVariable(name: "self", scope: !399, file: !364, line: 92, type: !206, align: 8)
!399 = !DILexicalBlockFile(scope: !391, file: !364, discriminator: 0)
!400 = !DILocation(line: 92, column: 21, scope: !368)
!401 = !DILocation(line: 89, column: 19, scope: !363)
!402 = !DILocalVariable(name: "self", scope: !403, file: !364, line: 89, type: !136, align: 8)
!403 = !DILexicalBlockFile(scope: !404, file: !364, discriminator: 0)
!404 = distinct !DISubprogram(name: "as_ptr<i8>", linkageName: "_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6as_ptr17h1be324413c9e8a4cE", scope: !193, file: !192, line: 476, type: !405, scopeLine: 476, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !407)
!405 = !DISubroutineType(types: !406)
!406 = !{!206, !136}
!407 = !{!402}
!408 = !DILocation(line: 89, column: 19, scope: !403, inlinedAt: !401)
!409 = !DILocation(line: 89, column: 13, scope: !368)
!410 = !DILocation(line: 92, column: 21, scope: !399, inlinedAt: !400)
!411 = !DILocalVariable(name: "self", scope: !412, file: !364, line: 92, type: !380, align: 8)
!412 = !DILexicalBlockFile(scope: !413, file: !364, discriminator: 0)
!413 = distinct !DISubprogram(name: "guaranteed_eq<u8>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$13guaranteed_eq17h142e7bb7efc55f0aE", scope: !393, file: !392, line: 777, type: !414, scopeLine: 777, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !383, retainedNodes: !416)
!414 = !DISubroutineType(types: !415)
!415 = !{!247, !380, !380}
!416 = !{!411, !417}
!417 = !DILocalVariable(name: "other", scope: !412, file: !364, line: 92, type: !380, align: 8)
!418 = !DILocation(line: 92, column: 21, scope: !412, inlinedAt: !419)
!419 = !DILocation(line: 39, column: 9, scope: !391, inlinedAt: !400)
!420 = !DILocation(line: 92, column: 21, scope: !421, inlinedAt: !428)
!421 = !DILexicalBlockFile(scope: !422, file: !364, discriminator: 0)
!422 = distinct !DISubprogram(name: "invalid<()>", linkageName: "_ZN4core3ptr7invalid17hc82b8e27796f75f0E", scope: !203, file: !275, line: 538, type: !423, scopeLine: 538, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !427, retainedNodes: !425)
!423 = !DISubroutineType(types: !424)
!424 = !{!6, !9}
!425 = !{!426}
!426 = !DILocalVariable(name: "addr", scope: !421, file: !364, line: 92, type: !9, align: 8)
!427 = !{!76}
!428 = !DILocation(line: 513, column: 20, scope: !387, inlinedAt: !390)
!429 = !DILocation(line: 92, column: 20, scope: !368)
!430 = !DILocation(line: 92, column: 13, scope: !368)
!431 = !DILocation(line: 94, column: 26, scope: !368)
!432 = !DILocation(line: 95, column: 17, scope: !368)
!433 = !DILocalVariable(name: "self", scope: !434, file: !364, line: 95, type: !380, align: 8)
!434 = !DILexicalBlockFile(scope: !435, file: !364, discriminator: 0)
!435 = distinct !DISubprogram(name: "wrapping_add<u8>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$12wrapping_add17h2cd5406be4f4c2dfE", scope: !393, file: !392, line: 1042, type: !436, scopeLine: 1042, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !383, retainedNodes: !438)
!436 = !DISubroutineType(types: !437)
!437 = !{!380, !380, !9}
!438 = !{!433, !439}
!439 = !DILocalVariable(name: "count", scope: !434, file: !364, line: 95, type: !9, align: 8)
!440 = !DILocation(line: 95, column: 17, scope: !434, inlinedAt: !432)
!441 = !DILocation(line: 95, column: 49, scope: !368)
!442 = !DILocalVariable(name: "self", scope: !443, file: !364, line: 95, type: !380, align: 8)
!443 = !DILexicalBlockFile(scope: !444, file: !364, discriminator: 0)
!444 = distinct !DISubprogram(name: "wrapping_offset<u8>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$15wrapping_offset17hc3b6119beb7c0952E", scope: !393, file: !392, line: 536, type: !445, scopeLine: 536, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !383, retainedNodes: !448)
!445 = !DISubroutineType(types: !446)
!446 = !{!380, !380, !447}
!447 = !DIBasicType(name: "isize", size: 64, encoding: DW_ATE_signed)
!448 = !{!442, !449}
!449 = !DILocalVariable(name: "count", scope: !443, file: !364, line: 95, type: !447, align: 8)
!450 = !DILocation(line: 95, column: 17, scope: !443, inlinedAt: !451)
!451 = !DILocation(line: 1046, column: 9, scope: !435, inlinedAt: !432)
!452 = !DILocation(line: 97, column: 17, scope: !368)
!453 = !DILocalVariable(name: "self", scope: !454, file: !364, line: 97, type: !206, align: 8)
!454 = !DILexicalBlockFile(scope: !455, file: !364, discriminator: 0)
!455 = distinct !DISubprogram(name: "add<i8>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$3add17h9f3dc51953195162E", scope: !393, file: !392, line: 871, type: !456, scopeLine: 871, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !458)
!456 = !DISubroutineType(types: !457)
!457 = !{!206, !206, !9}
!458 = !{!453, !459}
!459 = !DILocalVariable(name: "count", scope: !454, file: !364, line: 97, type: !9, align: 8)
!460 = !DILocation(line: 97, column: 17, scope: !454, inlinedAt: !452)
!461 = !DILocation(line: 97, column: 25, scope: !368)
!462 = !DILocalVariable(name: "self", scope: !463, file: !364, line: 97, type: !206, align: 8)
!463 = !DILexicalBlockFile(scope: !464, file: !364, discriminator: 0)
!464 = distinct !DISubprogram(name: "offset<i8>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$6offset17h668361163558eee3E", scope: !393, file: !392, line: 453, type: !465, scopeLine: 453, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !467)
!465 = !DISubroutineType(types: !466)
!466 = !{!206, !206, !447}
!467 = !{!462, !468}
!468 = !DILocalVariable(name: "count", scope: !463, file: !364, line: 97, type: !447, align: 8)
!469 = !DILocation(line: 97, column: 17, scope: !463, inlinedAt: !470)
!470 = !DILocation(line: 876, column: 18, scope: !455, inlinedAt: !452)
!471 = !DILocation(line: 94, column: 23, scope: !368)
!472 = !DILocation(line: 100, column: 48, scope: !370)
!473 = !DILocalVariable(name: "ptr", scope: !474, file: !364, line: 100, type: !479, align: 8)
!474 = !DILexicalBlockFile(scope: !475, file: !364, discriminator: 0)
!475 = distinct !DISubprogram(name: "new_unchecked<i8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17ha4fc9c63d5b089d6E", scope: !201, file: !476, line: 196, type: !477, scopeLine: 196, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !480)
!476 = !DIFile(filename: "/rustc/f9cba63746d0fff816250b2ba7b706b5d4dcf000/library/core/src/ptr/non_null.rs", directory: "", checksumkind: CSK_MD5, checksum: "856acc92efd7925b83dd1e3c577ddbdc")
!477 = !DISubroutineType(types: !478)
!478 = !{!201, !479}
!479 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut i8", baseType: !13, size: 64, align: 64, dwarfAddressSpace: 0)
!480 = !{!473}
!481 = !DILocation(line: 100, column: 25, scope: !474, inlinedAt: !482)
!482 = !DILocation(line: 100, column: 25, scope: !370)
!483 = !DILocation(line: 100, column: 64, scope: !370)
!484 = !DILocation(line: 100, column: 13, scope: !370)
!485 = !DILocation(line: 102, column: 6, scope: !363)
!486 = distinct !DISubprogram(name: "fmt<core::ffi::c_void>", linkageName: "_ZN50_$LT$$BP$mut$u20$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h8c2be5a5f4e50655E", scope: !487, file: !62, line: 2531, type: !488, scopeLine: 2531, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !493, retainedNodes: !490)
!487 = !DINamespace(name: "{impl#25}", scope: !54)
!488 = !DISubroutineType(types: !489)
!489 = !{!66, !35, !85}
!490 = !{!491, !492}
!491 = !DILocalVariable(name: "self", arg: 1, scope: !486, file: !62, line: 2531, type: !35)
!492 = !DILocalVariable(name: "f", arg: 2, scope: !486, file: !62, line: 2531, type: !85)
!493 = !{!494}
!494 = !DITemplateTypeParameter(name: "T", type: !37)
!495 = !DILocation(line: 2531, column: 12, scope: !486)
!496 = !DILocation(line: 2531, column: 19, scope: !486)
!497 = !DILocation(line: 2532, column: 9, scope: !486)
!498 = !DILocation(line: 2533, column: 6, scope: !486)
!499 = distinct !DISubprogram(name: "fmt<core::ffi::c_void>", linkageName: "_ZN52_$LT$$BP$mut$u20$T$u20$as$u20$core..fmt..Pointer$GT$3fmt17h3086c8efd06e963aE", scope: !500, file: !62, line: 2502, type: !488, scopeLine: 2502, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !493, retainedNodes: !501)
!500 = !DINamespace(name: "{impl#21}", scope: !54)
!501 = !{!502, !503}
!502 = !DILocalVariable(name: "self", arg: 1, scope: !499, file: !62, line: 2502, type: !35)
!503 = !DILocalVariable(name: "f", arg: 2, scope: !499, file: !62, line: 2502, type: !85)
!504 = !DILocation(line: 2502, column: 12, scope: !499)
!505 = !DILocation(line: 2502, column: 19, scope: !499)
!506 = !DILocation(line: 2503, column: 24, scope: !499)
!507 = !DILocation(line: 2503, column: 9, scope: !499)
!508 = !DILocation(line: 2504, column: 6, scope: !499)
!509 = distinct !DISubprogram(name: "fmt<core::ffi::c_void>", linkageName: "_ZN54_$LT$$BP$const$u20$T$u20$as$u20$core..fmt..Pointer$GT$3fmt17h5ce58e97e8216fadE", scope: !510, file: !62, line: 2462, type: !511, scopeLine: 2462, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !493, retainedNodes: !515)
!510 = !DINamespace(name: "{impl#20}", scope: !54)
!511 = !DISubroutineType(types: !512)
!512 = !{!66, !513, !85}
!513 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&*const core::ffi::c_void", baseType: !514, size: 64, align: 64, dwarfAddressSpace: 0)
!514 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const core::ffi::c_void", baseType: !37, size: 64, align: 64, dwarfAddressSpace: 0)
!515 = !{!516, !517}
!516 = !DILocalVariable(name: "self", arg: 1, scope: !509, file: !62, line: 2462, type: !513)
!517 = !DILocalVariable(name: "f", arg: 2, scope: !509, file: !62, line: 2462, type: !85)
!518 = !DILocation(line: 2462, column: 12, scope: !509)
!519 = !DILocation(line: 2462, column: 19, scope: !509)
!520 = !DILocation(line: 2464, column: 28, scope: !509)
!521 = !DILocation(line: 2464, column: 27, scope: !509)
!522 = !DILocalVariable(name: "self", scope: !523, file: !62, line: 2464, type: !6, align: 8)
!523 = !DILexicalBlockFile(scope: !524, file: !62, discriminator: 0)
!524 = distinct !DISubprogram(name: "addr<()>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$4addr17hc5c426e907d30be7E", scope: !393, file: !392, line: 178, type: !525, scopeLine: 178, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !427, retainedNodes: !527)
!525 = !DISubroutineType(types: !526)
!526 = !{!9, !6}
!527 = !{!522}
!528 = !DILocation(line: 2464, column: 27, scope: !523, inlinedAt: !521)
!529 = !DILocation(line: 2464, column: 9, scope: !509)
!530 = !DILocation(line: 2465, column: 6, scope: !509)
!531 = distinct !DISubprogram(name: "into_iter<core::slice::iter::Iter<i8>>", linkageName: "_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17hd26588b6111d0594E", scope: !533, file: !532, line: 271, type: !537, scopeLine: 271, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !541, retainedNodes: !539)
!532 = !DIFile(filename: "/rustc/f9cba63746d0fff816250b2ba7b706b5d4dcf000/library/core/src/iter/traits/collect.rs", directory: "", checksumkind: CSK_MD5, checksum: "73b4070628f6fcec74cccccb11afa9ef")
!533 = !DINamespace(name: "{impl#0}", scope: !534)
!534 = !DINamespace(name: "collect", scope: !535)
!535 = !DINamespace(name: "traits", scope: !536)
!536 = !DINamespace(name: "iter", scope: !39)
!537 = !DISubroutineType(types: !538)
!538 = !{!197, !197}
!539 = !{!540}
!540 = !DILocalVariable(name: "self", arg: 1, scope: !531, file: !532, line: 271, type: !197)
!541 = !{!260}
!542 = !DILocation(line: 271, column: 18, scope: !531)
!543 = !DILocation(line: 273, column: 6, scope: !531)
!544 = distinct !DISubprogram(name: "next<i8>", linkageName: "_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h71cf1614be9b7a2bE", scope: !546, file: !545, line: 134, type: !547, scopeLine: 134, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !561)
!545 = !DIFile(filename: "/rustc/f9cba63746d0fff816250b2ba7b706b5d4dcf000/library/core/src/slice/iter/macros.rs", directory: "", checksumkind: CSK_MD5, checksum: "41903112aef4079fb81d70967f52d92d")
!546 = !DINamespace(name: "{impl#173}", scope: !198)
!547 = !DISubroutineType(types: !548)
!548 = !{!549, !560}
!549 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<&i8>", scope: !95, file: !2, size: 64, align: 64, elements: !550, templateParams: !14, identifier: "b74e6dca1cab41ee6d7d599298d207a7")
!550 = !{!551}
!551 = !DICompositeType(tag: DW_TAG_variant_part, scope: !549, file: !2, size: 64, align: 64, elements: !552, templateParams: !14, identifier: "a4b0cf1aae4b0c89ab2e5e1a83358def", discriminator: !559)
!552 = !{!553, !555}
!553 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !551, file: !2, baseType: !554, size: 64, align: 64, extraData: i64 0)
!554 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !549, file: !2, size: 64, align: 64, elements: !14, templateParams: !211, identifier: "ceb781ad2ec2fa841d9f58d7253453be")
!555 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !551, file: !2, baseType: !556, size: 64, align: 64)
!556 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !549, file: !2, size: 64, align: 64, elements: !557, templateParams: !211, identifier: "d2e679bd0f8fac5f47132283ea6ceb76")
!557 = !{!558}
!558 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !556, file: !2, baseType: !12, size: 64, align: 64)
!559 = !DIDerivedType(tag: DW_TAG_member, scope: !549, file: !2, baseType: !108, size: 64, align: 64, flags: DIFlagArtificial)
!560 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::slice::iter::Iter<i8>", baseType: !197, size: 64, align: 64, dwarfAddressSpace: 0)
!561 = !{!562}
!562 = !DILocalVariable(name: "self", arg: 1, scope: !544, file: !545, line: 134, type: !560)
!563 = !DILocation(line: 134, column: 21, scope: !544)
!564 = !DILocalVariable(name: "metadata", scope: !565, file: !545, line: 142, type: !7, align: 1)
!565 = !DILexicalBlockFile(scope: !566, file: !545, discriminator: 0)
!566 = distinct !DISubprogram(name: "from_raw_parts_mut<u8>", linkageName: "_ZN4core3ptr8metadata18from_raw_parts_mut17hb6e1d242264ce342E", scope: !377, file: !376, line: 127, type: !567, scopeLine: 127, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !383, retainedNodes: !571)
!567 = !DISubroutineType(types: !568)
!568 = !{!569, !570, !7}
!569 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut u8", baseType: !40, size: 64, align: 64, dwarfAddressSpace: 0)
!570 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut ()", baseType: !7, size: 64, align: 64, dwarfAddressSpace: 0)
!571 = !{!572, !564}
!572 = !DILocalVariable(name: "data_address", scope: !565, file: !545, line: 142, type: !570, align: 8)
!573 = !DILocation(line: 142, column: 29, scope: !565, inlinedAt: !574)
!574 = !DILocation(line: 668, column: 5, scope: !575, inlinedAt: !578)
!575 = distinct !DISubprogram(name: "null_mut<u8>", linkageName: "_ZN4core3ptr8null_mut17h03653357030ea5b6E", scope: !203, file: !275, line: 667, type: !576, scopeLine: 667, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !383, retainedNodes: !14)
!576 = !DISubroutineType(types: !577)
!577 = !{!569}
!578 = !DILocation(line: 38, column: 41, scope: !579, inlinedAt: !588)
!579 = distinct !DISubprogram(name: "is_null<i8>", linkageName: "_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17h6cdb0b5486e61bb1E", scope: !581, file: !580, line: 35, type: !583, scopeLine: 35, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !585)
!580 = !DIFile(filename: "/rustc/f9cba63746d0fff816250b2ba7b706b5d4dcf000/library/core/src/ptr/mut_ptr.rs", directory: "", checksumkind: CSK_MD5, checksum: "d1207c8d2ff8d49dc119277a40217f3e")
!581 = !DINamespace(name: "{impl#0}", scope: !582)
!582 = !DINamespace(name: "mut_ptr", scope: !203)
!583 = !DISubroutineType(types: !584)
!584 = !{!247, !479}
!585 = !{!586}
!586 = !DILocalVariable(name: "self", scope: !587, file: !545, line: 142, type: !479, align: 8)
!587 = !DILexicalBlockFile(scope: !579, file: !545, discriminator: 0)
!588 = !DILocation(line: 142, column: 29, scope: !544)
!589 = !DILocalVariable(name: "metadata", scope: !590, file: !545, line: 144, type: !7, align: 1)
!590 = !DILexicalBlockFile(scope: !591, file: !545, discriminator: 0)
!591 = distinct !DISubprogram(name: "from_raw_parts<u8>", linkageName: "_ZN4core3ptr8metadata14from_raw_parts17h7d57f7befca6c5abE", scope: !377, file: !376, line: 110, type: !378, scopeLine: 110, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !383, retainedNodes: !592)
!592 = !{!593, !589}
!593 = !DILocalVariable(name: "data_address", scope: !590, file: !545, line: 144, type: !6, align: 8)
!594 = !DILocation(line: 144, column: 33, scope: !590, inlinedAt: !595)
!595 = !DILocation(line: 513, column: 5, scope: !596, inlinedAt: !597)
!596 = distinct !DISubprogram(name: "null<u8>", linkageName: "_ZN4core3ptr4null17habef1b51f6ef211dE", scope: !203, file: !275, line: 512, type: !388, scopeLine: 512, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !383, retainedNodes: !14)
!597 = !DILocation(line: 39, column: 43, scope: !598, inlinedAt: !602)
!598 = distinct !DISubprogram(name: "is_null<i8>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7is_null17h6244b6be9c3b154eE", scope: !393, file: !392, line: 36, type: !395, scopeLine: 36, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !599)
!599 = !{!600}
!600 = !DILocalVariable(name: "self", scope: !601, file: !545, line: 144, type: !206, align: 8)
!601 = !DILexicalBlockFile(scope: !598, file: !545, discriminator: 0)
!602 = !DILocation(line: 144, column: 33, scope: !544)
!603 = !DILocalVariable(name: "self", scope: !604, file: !545, line: 142, type: !201, align: 8)
!604 = !DILexicalBlockFile(scope: !605, file: !545, discriminator: 0)
!605 = distinct !DISubprogram(name: "as_ptr<i8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h6a82a5c8753c9f44E", scope: !201, file: !476, line: 330, type: !606, scopeLine: 330, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !608)
!606 = !DISubroutineType(types: !607)
!607 = !{!479, !201}
!608 = !{!603}
!609 = !DILocation(line: 142, column: 29, scope: !604, inlinedAt: !588)
!610 = !DILocation(line: 142, column: 29, scope: !587, inlinedAt: !588)
!611 = !DILocalVariable(name: "self", scope: !612, file: !545, line: 142, type: !569, align: 8)
!612 = !DILexicalBlockFile(scope: !613, file: !545, discriminator: 0)
!613 = distinct !DISubprogram(name: "guaranteed_eq<u8>", linkageName: "_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$13guaranteed_eq17h61de1368161b9887E", scope: !581, file: !580, line: 707, type: !614, scopeLine: 707, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !383, retainedNodes: !616)
!614 = !DISubroutineType(types: !615)
!615 = !{!247, !569, !569}
!616 = !{!611, !617}
!617 = !DILocalVariable(name: "other", scope: !612, file: !545, line: 142, type: !569, align: 8)
!618 = !DILocation(line: 142, column: 29, scope: !612, inlinedAt: !619)
!619 = !DILocation(line: 38, column: 9, scope: !579, inlinedAt: !588)
!620 = !DILocation(line: 142, column: 29, scope: !621, inlinedAt: !627)
!621 = !DILexicalBlockFile(scope: !622, file: !545, discriminator: 0)
!622 = distinct !DISubprogram(name: "invalid_mut<()>", linkageName: "_ZN4core3ptr11invalid_mut17hcf81244d4676ee03E", scope: !203, file: !275, line: 569, type: !623, scopeLine: 569, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !427, retainedNodes: !625)
!623 = !DISubroutineType(types: !624)
!624 = !{!570, !9}
!625 = !{!626}
!626 = !DILocalVariable(name: "addr", scope: !621, file: !545, line: 142, type: !9, align: 8)
!627 = !DILocation(line: 668, column: 24, scope: !575, inlinedAt: !578)
!628 = !DILocation(line: 142, column: 28, scope: !544)
!629 = !DILocation(line: 142, column: 21, scope: !544)
!630 = !DILocation(line: 143, column: 24, scope: !544)
!631 = !DILocation(line: 143, column: 21, scope: !544)
!632 = !DILocation(line: 144, column: 33, scope: !601, inlinedAt: !602)
!633 = !DILocalVariable(name: "self", scope: !634, file: !545, line: 144, type: !380, align: 8)
!634 = !DILexicalBlockFile(scope: !635, file: !545, discriminator: 0)
!635 = distinct !DISubprogram(name: "guaranteed_eq<u8>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$13guaranteed_eq17h142e7bb7efc55f0aE", scope: !393, file: !392, line: 777, type: !414, scopeLine: 777, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !383, retainedNodes: !636)
!636 = !{!633, !637}
!637 = !DILocalVariable(name: "other", scope: !634, file: !545, line: 144, type: !380, align: 8)
!638 = !DILocation(line: 144, column: 33, scope: !634, inlinedAt: !639)
!639 = !DILocation(line: 39, column: 9, scope: !598, inlinedAt: !602)
!640 = !DILocation(line: 144, column: 33, scope: !641, inlinedAt: !645)
!641 = !DILexicalBlockFile(scope: !642, file: !545, discriminator: 0)
!642 = distinct !DISubprogram(name: "invalid<()>", linkageName: "_ZN4core3ptr7invalid17hc82b8e27796f75f0E", scope: !203, file: !275, line: 538, type: !423, scopeLine: 538, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !427, retainedNodes: !643)
!643 = !{!644}
!644 = !DILocalVariable(name: "addr", scope: !641, file: !545, line: 144, type: !9, align: 8)
!645 = !DILocation(line: 513, column: 20, scope: !596, inlinedAt: !597)
!646 = !DILocation(line: 144, column: 32, scope: !544)
!647 = !DILocation(line: 144, column: 25, scope: !544)
!648 = !DILocation(line: 146, column: 24, scope: !544)
!649 = !DILocalVariable(name: "self", scope: !650, file: !545, line: 146, type: !201, align: 8)
!650 = !DILexicalBlockFile(scope: !651, file: !545, discriminator: 0)
!651 = distinct !DISubprogram(name: "as_ptr<i8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h6a82a5c8753c9f44E", scope: !201, file: !476, line: 330, type: !606, scopeLine: 330, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !652)
!652 = !{!649}
!653 = !DILocation(line: 146, column: 24, scope: !650, inlinedAt: !654)
!654 = !DILocation(line: 8, column: 9, scope: !544)
!655 = !DILocation(line: 149, column: 30, scope: !544)
!656 = !DILocalVariable(name: "self", scope: !657, file: !545, line: 149, type: !560, align: 8)
!657 = distinct !DISubprogram(name: "post_inc_start<i8>", linkageName: "_ZN4core5slice4iter13Iter$LT$T$GT$14post_inc_start17h2292d9e09125b36eE", scope: !197, file: !545, line: 85, type: !658, scopeLine: 85, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !660)
!658 = !DISubroutineType(types: !659)
!659 = !{!206, !560, !447}
!660 = !{!656, !661, !662}
!661 = !DILocalVariable(name: "offset", scope: !657, file: !545, line: 149, type: !447, align: 8)
!662 = !DILocalVariable(name: "old", scope: !663, file: !545, line: 149, type: !479, align: 8)
!663 = distinct !DILexicalBlock(scope: !657, file: !545, line: 90, column: 21)
!664 = !DILocation(line: 149, column: 30, scope: !657, inlinedAt: !665)
!665 = !DILocation(line: 53, column: 47, scope: !544)
!666 = !DILocation(line: 147, column: 25, scope: !544)
!667 = !DILocation(line: 146, column: 21, scope: !544)
!668 = !DILocation(line: 152, column: 14, scope: !544)
!669 = !DILocalVariable(name: "self", scope: !670, file: !545, line: 149, type: !380, align: 8)
!670 = !DILexicalBlockFile(scope: !671, file: !545, discriminator: 0)
!671 = distinct !DISubprogram(name: "wrapping_offset<u8>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$15wrapping_offset17hc3b6119beb7c0952E", scope: !393, file: !392, line: 536, type: !445, scopeLine: 536, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !383, retainedNodes: !672)
!672 = !{!669, !673}
!673 = !DILocalVariable(name: "count", scope: !670, file: !545, line: 149, type: !447, align: 8)
!674 = !DILocation(line: 149, column: 30, scope: !670, inlinedAt: !675)
!675 = !DILocation(line: 67, column: 29, scope: !657, inlinedAt: !665)
!676 = !DILocalVariable(name: "self", scope: !677, file: !545, line: 149, type: !201, align: 8)
!677 = !DILexicalBlockFile(scope: !678, file: !545, discriminator: 0)
!678 = distinct !DISubprogram(name: "as_ptr<i8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h6a82a5c8753c9f44E", scope: !201, file: !476, line: 330, type: !606, scopeLine: 330, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !679)
!679 = !{!676}
!680 = !DILocation(line: 149, column: 30, scope: !677, inlinedAt: !681)
!681 = !DILocation(line: 90, column: 31, scope: !657, inlinedAt: !665)
!682 = !DILocation(line: 149, column: 30, scope: !663, inlinedAt: !665)
!683 = !DILocalVariable(name: "self", scope: !684, file: !545, line: 149, type: !201, align: 8)
!684 = !DILexicalBlockFile(scope: !685, file: !545, discriminator: 0)
!685 = distinct !DISubprogram(name: "as_ptr<i8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h6a82a5c8753c9f44E", scope: !201, file: !476, line: 330, type: !606, scopeLine: 330, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !686)
!686 = !{!683}
!687 = !DILocation(line: 149, column: 30, scope: !684, inlinedAt: !688)
!688 = !DILocation(line: 93, column: 64, scope: !663, inlinedAt: !665)
!689 = !DILocalVariable(name: "self", scope: !690, file: !545, line: 149, type: !479, align: 8)
!690 = !DILexicalBlockFile(scope: !691, file: !545, discriminator: 0)
!691 = distinct !DISubprogram(name: "offset<i8>", linkageName: "_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$6offset17h955a6243b9a54ab6E", scope: !581, file: !580, line: 465, type: !692, scopeLine: 465, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !694)
!692 = !DISubroutineType(types: !693)
!693 = !{!479, !479, !447}
!694 = !{!689, !695}
!695 = !DILocalVariable(name: "count", scope: !690, file: !545, line: 149, type: !447, align: 8)
!696 = !DILocation(line: 149, column: 30, scope: !690, inlinedAt: !688)
!697 = !DILocalVariable(name: "ptr", scope: !698, file: !545, line: 149, type: !479, align: 8)
!698 = !DILexicalBlockFile(scope: !699, file: !545, discriminator: 0)
!699 = distinct !DISubprogram(name: "new_unchecked<i8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17ha4fc9c63d5b089d6E", scope: !201, file: !476, line: 196, type: !477, scopeLine: 196, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !700)
!700 = !{!697}
!701 = !DILocation(line: 149, column: 30, scope: !698, inlinedAt: !702)
!702 = !DILocation(line: 93, column: 41, scope: !663, inlinedAt: !665)
!703 = !DILocation(line: 149, column: 25, scope: !544)
!704 = !DILocalVariable(name: "self", scope: !705, file: !545, line: 149, type: !201, align: 8)
!705 = !DILexicalBlockFile(scope: !706, file: !545, discriminator: 0)
!706 = distinct !DISubprogram(name: "as_ptr<i8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h6a82a5c8753c9f44E", scope: !201, file: !476, line: 330, type: !606, scopeLine: 330, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !144, retainedNodes: !707)
!707 = !{!704}
!708 = !DILocation(line: 149, column: 30, scope: !705, inlinedAt: !709)
!709 = !DILocation(line: 88, column: 21, scope: !657, inlinedAt: !665)
!710 = distinct !DISubprogram(name: "sayHello", linkageName: "_ZN3sys5Hello8sayHello17hc58b692bab97984bE", scope: !712, file: !711, line: 22, type: !717, scopeLine: 22, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !48, templateParams: !14, retainedNodes: !720)
!711 = !DIFile(filename: "sys/src/../csrc/bindings.rs", directory: "/home/ubuntu/Workspace/RustCLA/ABIChecker/tests/wstest", checksumkind: CSK_MD5, checksum: "2eeb820f7d641c71e087dac32daf1698")
!712 = !DICompositeType(tag: DW_TAG_structure_type, name: "Hello", scope: !713, file: !2, size: 192, align: 64, elements: !714, templateParams: !14, identifier: "c8eddfd74cbee4a9b0862492ef37f0b1")
!713 = !DINamespace(name: "sys", scope: null)
!714 = !{!715, !716}
!715 = !DIDerivedType(tag: DW_TAG_member, name: "data", scope: !712, file: !2, baseType: !24, size: 128, align: 8)
!716 = !DIDerivedType(tag: DW_TAG_member, name: "ptr", scope: !712, file: !2, baseType: !36, size: 64, align: 64, offset: 128)
!717 = !DISubroutineType(types: !718)
!718 = !{null, !719}
!719 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut sys::Hello", baseType: !712, size: 64, align: 64, dwarfAddressSpace: 0)
!720 = !{!721}
!721 = !DILocalVariable(name: "self", arg: 1, scope: !710, file: !711, line: 22, type: !719)
!722 = !DILocation(line: 22, column: 28, scope: !710)
!723 = !DILocation(line: 23, column: 9, scope: !710)
!724 = !DILocation(line: 24, column: 6, scope: !710)
!725 = distinct !DISubprogram(name: "new", linkageName: "_ZN3sys5Hello3new17h3c7bea36bb2a3dc0E", scope: !712, file: !711, line: 25, type: !726, scopeLine: 25, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !48, templateParams: !14, retainedNodes: !728)
!726 = !DISubroutineType(types: !727)
!727 = !{!712}
!728 = !{!729}
!729 = !DILocalVariable(name: "__bindgen_tmp", scope: !730, file: !711, line: 26, type: !731, align: 8)
!730 = distinct !DILexicalBlock(scope: !725, file: !711, line: 26, column: 9)
!731 = !DICompositeType(tag: DW_TAG_union_type, name: "MaybeUninit<sys::Hello>", scope: !732, file: !2, size: 192, align: 64, elements: !734, templateParams: !741, identifier: "e28c174c79d0909db0b87000e62c09f3")
!732 = !DINamespace(name: "maybe_uninit", scope: !733)
!733 = !DINamespace(name: "mem", scope: !39)
!734 = !{!735, !736}
!735 = !DIDerivedType(tag: DW_TAG_member, name: "uninit", scope: !731, file: !2, baseType: !7, align: 8)
!736 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !731, file: !2, baseType: !737, size: 192, align: 64)
!737 = !DICompositeType(tag: DW_TAG_structure_type, name: "ManuallyDrop<sys::Hello>", scope: !738, file: !2, size: 192, align: 64, elements: !739, templateParams: !741, identifier: "2d7a6107676df9f0eef115a39028e4f5")
!738 = !DINamespace(name: "manually_drop", scope: !733)
!739 = !{!740}
!740 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !737, file: !2, baseType: !712, size: 192, align: 64)
!741 = !{!742}
!742 = !DITemplateTypeParameter(name: "T", type: !712)
!743 = !DILocation(line: 26, column: 13, scope: !730)
!744 = !DILocation(line: 321, column: 9, scope: !745, inlinedAt: !749)
!745 = distinct !DISubprogram(name: "uninit<sys::Hello>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$6uninit17h2c0590f2d55b672cE", scope: !731, file: !746, line: 320, type: !747, scopeLine: 320, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !741, retainedNodes: !14)
!746 = !DIFile(filename: "/rustc/f9cba63746d0fff816250b2ba7b706b5d4dcf000/library/core/src/mem/maybe_uninit.rs", directory: "", checksumkind: CSK_MD5, checksum: "d75c93d9a93fe65ef19edc0e8e272278")
!747 = !DISubroutineType(types: !748)
!748 = !{!731}
!749 = distinct !DILocation(line: 26, column: 33, scope: !725)
!750 = !DILocation(line: 26, column: 33, scope: !725)
!751 = !DILocalVariable(name: "self", arg: 1, scope: !752, file: !746, line: 574, type: !756)
!752 = distinct !DISubprogram(name: "as_mut_ptr<sys::Hello>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$10as_mut_ptr17h4051e3086fafc38cE", scope: !731, file: !746, line: 574, type: !753, scopeLine: 574, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !741, retainedNodes: !757)
!753 = !DISubroutineType(types: !754)
!754 = !{!755, !756}
!755 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut sys::Hello", baseType: !712, size: 64, align: 64, dwarfAddressSpace: 0)
!756 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::mem::maybe_uninit::MaybeUninit<sys::Hello>", baseType: !731, size: 64, align: 64, dwarfAddressSpace: 0)
!757 = !{!751}
!758 = !DILocation(line: 574, column: 29, scope: !752, inlinedAt: !759)
!759 = distinct !DILocation(line: 27, column: 21, scope: !730)
!760 = !DILocation(line: 576, column: 9, scope: !752, inlinedAt: !759)
!761 = !DILocation(line: 27, column: 21, scope: !730)
!762 = !DILocation(line: 27, column: 9, scope: !730)
!763 = !DILocation(line: 28, column: 9, scope: !730)
!764 = !DILocalVariable(name: "self", arg: 1, scope: !765, file: !746, line: 629, type: !731)
!765 = distinct !DISubprogram(name: "assume_init<sys::Hello>", linkageName: "_ZN4core3mem12maybe_uninit20MaybeUninit$LT$T$GT$11assume_init17h6a629773d2c7c918E", scope: !731, file: !746, line: 629, type: !766, scopeLine: 629, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !741, retainedNodes: !768)
!766 = !DISubroutineType(types: !767)
!767 = !{!712, !731, !319}
!768 = !{!764}
!769 = !DILocation(line: 629, column: 37, scope: !765, inlinedAt: !770)
!770 = distinct !DILocation(line: 28, column: 9, scope: !730)
!771 = !DILocalVariable(name: "slot", scope: !772, file: !746, line: 634, type: !737, align: 8)
!772 = !DILexicalBlockFile(scope: !773, file: !746, discriminator: 0)
!773 = distinct !DISubprogram(name: "into_inner<sys::Hello>", linkageName: "_ZN4core3mem13manually_drop21ManuallyDrop$LT$T$GT$10into_inner17h9cf4d2c66e3f3700E", scope: !737, file: !774, line: 88, type: !775, scopeLine: 88, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !48, templateParams: !741, retainedNodes: !777)
!774 = !DIFile(filename: "/rustc/f9cba63746d0fff816250b2ba7b706b5d4dcf000/library/core/src/mem/manually_drop.rs", directory: "", checksumkind: CSK_MD5, checksum: "a34f39f0c1c25c8378cd3f4ec0bf00c3")
!775 = !DISubroutineType(types: !776)
!776 = !{!712, !737}
!777 = !{!771}
!778 = !DILocation(line: 634, column: 13, scope: !772, inlinedAt: !779)
!779 = distinct !DILocation(line: 634, column: 13, scope: !765, inlinedAt: !770)
!780 = !DILocation(line: 634, column: 38, scope: !765, inlinedAt: !770)
!781 = !DILocation(line: 29, column: 6, scope: !725)
!782 = distinct !DISubprogram(name: "destruct", linkageName: "_ZN3sys5Hello8destruct17h5ed162002c7eb0c2E", scope: !712, file: !711, line: 30, type: !717, scopeLine: 30, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !48, templateParams: !14, retainedNodes: !783)
!783 = !{!784}
!784 = !DILocalVariable(name: "self", arg: 1, scope: !782, file: !711, line: 30, type: !719)
!785 = !DILocation(line: 30, column: 28, scope: !782)
!786 = !DILocation(line: 31, column: 9, scope: !782)
!787 = !DILocation(line: 32, column: 6, scope: !782)
!788 = distinct !DISubprogram(name: "fmt", linkageName: "_ZN47_$LT$sys..Hello$u20$as$u20$core..fmt..Debug$GT$3fmt17h1a8253c24512449bE", scope: !789, file: !711, line: 4, type: !790, scopeLine: 4, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !48, templateParams: !14, retainedNodes: !793)
!789 = !DINamespace(name: "{impl#1}", scope: !713)
!790 = !DISubroutineType(types: !791)
!791 = !{!66, !792, !85}
!792 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&sys::Hello", baseType: !712, size: 64, align: 64, dwarfAddressSpace: 0)
!793 = !{!794, !795}
!794 = !DILocalVariable(name: "self", arg: 1, scope: !788, file: !711, line: 4, type: !792)
!795 = !DILocalVariable(name: "f", arg: 2, scope: !788, file: !711, line: 4, type: !85)
!796 = !DILocation(line: 4, column: 10, scope: !788)
!797 = !DILocation(line: 6, column: 5, scope: !788)
!798 = !DILocation(line: 7, column: 5, scope: !788)
!799 = !DILocation(line: 4, column: 15, scope: !788)
