FROM rust:1.70

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/fast-test-rust"]