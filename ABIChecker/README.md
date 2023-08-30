# ABI Checker


## Requirements

```sh
sudo apt install -y llvm-15 clang-15 lld-15 libclang-15-dev libpolly-15-dev
rustup override set nightly-2022-10-01 # This rustc uses LLVM-15
```

**Notice:**
- Because we're utilizing LLVM-15, it's important to note that your Rustc and Clang versions need to align accordingly. In other words, the LLVM edition within your Rustc and Clang must be 15 or lower.
- Your LLVM tools, Rustc, Clang, and so on, should be named using their original names, without version suffixes. This is because when installing them with tools like apt, they typically include version suffixes, which can lead to path issues.

## Build
```sh
cargo build
```

## Usage

checkout to directory `tests/simpletest` and execute
```sh
cargo run --manifest-path ../../Cargo.toml
```

If you want to use it for your own project, Pplease add binary file `abi_checker` to your environment path.
```sh
# in the root of projects, run
$ abi_checker
```

## File Architecture
```c
.
├── collector       // lib to parse c & rust codes
├── modified_deps   // modified bindfen and cbindgen
│   ├── bindgen
│   └── cbindgen
├── src
│   ├── analysis    // analyze result
│   └── collect     // collect c & rust & ir codes
└── tests
    └── simpletest
```

## Use docker

Refer to https://docs.docker.com/desktop/install/ubuntu/ to install docker.

```Shell
# Build Docker
docker build -t abi-checker .
# Run Docker
docker run -it -w /app --mount type=bind,src="$(pwd)",target=/app abi-checker bash

# Exec into the docker shell
docker exec -it <docker-id> bash
# Now, you can feel free to other README and build tools. 
```
