# ABI Checker


## Requirements

```sh
sudo apt install -y llvm-15 clang-15 lld-15 libclang-15-dev libpolly-15-dev
rustup override set nightly-2022-10-01
```

## Build
```sh
cargo build
```

## Usage

checkout to directory `tests/simpletest` and execute
```sh
cargo run --manifest-path ../../Cargo.toml
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

```Shell
# Build Docker
docker build -t abi-checker .
# Run Docker
docker run -it -w /app --mount type=bind,src="$(pwd)",target=/app abi-checker bash

# Exec into the docker shell
docker exec -it <docker-id> bash
# Now, you can feel free to other README and build tools. 
```
