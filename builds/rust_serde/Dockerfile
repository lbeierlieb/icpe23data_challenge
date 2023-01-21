FROM rust:1.66.0
COPY implementations/rust_serde/Cargo.toml .
COPY implementations/rust_serde/Cargo.lock .
COPY implementations/rust_serde/src/main.rs src/
RUN cargo install --path .
CMD ["rust_serde"]
