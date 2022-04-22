FROM rust

COPY ./target/release/vossl /bin/vossl

CMD ["/bin/vossl"]
