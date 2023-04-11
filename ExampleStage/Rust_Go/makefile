run: all
	GODEBUG='gctrace=1' ./main

all: go_static
	rustc main.rs -l hello -L .

go_static:
	go build -buildmode=c-archive -o libhello.a hello.go

clean:
	@rm -rf *.o *.a *.h main