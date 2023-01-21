FROM rust:1.66.0
ENV RAYON_NUM_THREADS=4
COPY implementations/rust_serde_rayon/Cargo.toml .
COPY implementations/rust_serde_rayon/Cargo.lock .
COPY implementations/rust_serde_rayon/src/main.rs src/
RUN cargo install --path .
CMD ["rust_serde_rayon"]
