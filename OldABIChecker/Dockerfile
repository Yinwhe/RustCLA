FROM ubuntu

RUN apt-get update && apt-get install -y make gcc
RUN apt-get install -y build-essential pkg-config libssl-dev
RUN apt-get install -y cmake curl vim python3 git pip zip
RUN apt-get install -y llvm-15 clang-15 lld-15 libclang-15-dev clang
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH=$PATH:/root/.cargo/bin
ENV PATH=$PATH:/usr/lib/llvm-15/bin
# Verify version
RUN rustc --version && cargo --version
# Switch to rustc that uses LLVM-15
RUN rustup override set nightly-2022-10-01

# CMD ["make", "postgresql"]
CMD ["sh", "-c", "while true; do sleep 1; done"]