# ABI Checker


## Requirements

```sh
sudo apt install llvm-15
sudo apt install clang-15
sudo apt install lld-15
sudo apt install libclang-15-dev
```

## Build
```sh
cargo build
```

## Usage


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