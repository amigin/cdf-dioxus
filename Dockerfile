FROM ubuntu:22.04

COPY ./target/release/web-terminal ./target/release/web-terminal
COPY ./files ./target/release/files
ENTRYPOINT ["./target/release/web-terminal"]