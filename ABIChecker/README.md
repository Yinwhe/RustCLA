# ABI Checker


## Requirements

```sh
sudo apt install llvm-15
sudo apt install lld-15
sudo apt install libclang-15-dev
```

**Notice:**
- Because we're utilizing LLVM-15, it's important to note that your Rustc and Clang versions need to align accordingly. In other words, the LLVM edition within your Rustc and Clang must be 15 or lower.
- Your LLVM tools, Rustc, Clang, and so on, should be named using their original names, without version suffixes. This is because when installing them with tools like apt, they typically include version suffixes, which can lead to path issues.

## Build
```sh
cargo build
```

## Usage
Please add `abi_checker` to your environment path.
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