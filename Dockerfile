FROM rust as builder

WORKDIR /source
COPY . /source

RUN cargo build --release

FROM debian:bookworm as runner
COPY --from=builder /source/target/release/tenjin /

ENTRYPOINT [ "/tenjin" ]

