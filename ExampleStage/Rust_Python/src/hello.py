import ctypes

def hello():
    print("Hello World!")

def read_val(ptr):
    arr = (ctypes.c_int64 * 4).from_address(ptr)
    arr[3] = 0xdeadbeef

def read_box(ptr):
    ptr = ctypes.cast(ptr, ctypes.POINTER(ctypes.c_int64))
    ctypes.CDLL("libc.so.6").free(ptr)

def test(ptr):
    ptr = ctypes.cast(ptr, ctypes.POINTER(ctypes.c_int64))
    ctypes.CDLL("libc.so.6").free(ptr)