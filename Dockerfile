FROM ubuntu:20.04 as builder

RUN apt-get update && \
    apt-get install -y libssl-dev

FROM rust:slim

# Copy OpenSSL libraries from Ubuntu 20.04 image
COPY --from=builder /usr/lib/x86_64-linux-gnu/libssl.so.1.1 /usr/lib/x86_64-linux-gnu/libssl.so.1.1
COPY --from=builder /usr/lib/x86_64-linux-gnu/libcrypto.so.1.1 /usr/lib/x86_64-linux-gnu/libcrypto.so.1.1

COPY ./target/release/web-terminal ./target/release/web-terminal
ENTRYPOINT ["./target/release/web-terminal"]