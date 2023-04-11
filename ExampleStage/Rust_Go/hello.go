package main

/*
#include <stdio.h>
#include <stdlib.h>

void hello();
*/
import "C"
import (
	"fmt"
	"reflect"
	"runtime"
	"unsafe"
)

func main() {}

//export hello
func hello() {
	fmt.Println("Hello Rust! We hacked it!")
}

//export read_val
func read_val(x *C.ulonglong) {
	// overwrite
	ptr := (*[4]uint64)(unsafe.Pointer(x))
	ptr[3] = (uint64)(reflect.ValueOf(C.hello).Pointer())
}

//export read_box
func read_box(x *C.ulonglong) {
	xx := int64(*x)
	// double free or corruption (out)
	defer C.free(unsafe.Pointer(x)) // normal operation

	// do something
	xx += 1
}

//export go_alloc
func go_alloc() uintptr {
	mem := make([]uint64, 32*2000*2000)
	for i := 0; i < 32*2000*2000; i++ {
		mem[i] = (uint64)(i)
	}

	return uintptr(unsafe.Pointer(&mem[0]))
}

//export force_gc
func force_gc() {
	runtime.GC()
}
