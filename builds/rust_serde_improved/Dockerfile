FROM rust:1.66.0
COPY implementations/rust_serde_improved/Cargo.toml .
COPY implementations/rust_serde_improved/Cargo.lock .
COPY implementations/rust_serde_improved/src/main.rs src/
# ARG RUSTFLAGS="-C target-cpu=native"
RUN cargo install --path .
CMD ["rust_serde_improved"]
