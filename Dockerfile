FROM rust:slim

RUN apt-get install libssl-dev -y

COPY ./target/release/web-terminal ./target/release/web-terminal
COPY ./files ./target/release/files
ENTRYPOINT ["./target/release/web-terminal"]