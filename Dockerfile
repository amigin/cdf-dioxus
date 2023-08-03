FROM rust:slim
COPY ./target/release/web-terminal ./target/release/web-terminal
COPY ./files ./files 
RUN export LD_LIBRARY_PATH = /usr/local/lib64:$LD_LIBRARY_PATH
ENTRYPOINT ["./target/release/web-terminal"]