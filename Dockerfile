FROM rust:slim
RUN apt-get update && apt-get install -y libssl-dev
RUN ln -s /usr/lib/x86_64-linux-gnu/libssl.so.1.0.0 /usr/lib/x86_64-linux-gnu/libssl.so.3
COPY ./target/release/web-terminal ./target/release/web-terminal
COPY ./files ./files 
ENV LD_LIBRARY_PATH=/usr/local/lib64:$LD_LIBRARY_PATH
ENTRYPOINT ["./target/release/web-terminal"]