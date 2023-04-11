run: all
	./main

all: c_static
	rustc main.rs -l hello -L . -Clink-arg=-fuse-ld=lld

c_static:
	clang hello.c -c -flto -fvisibility=hidden -fsanitize=cfi -fPIC
	llvm-ar rsc libhello.a hello.o

clean:
	@rm -rf *.o *.a main