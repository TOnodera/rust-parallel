FROM rust:latest

RUN useradd -m -u 1000 -s /bin/bash rust 
RUN rustup component add rustfmt
RUN apt update -y && apt install telnet
USER rust