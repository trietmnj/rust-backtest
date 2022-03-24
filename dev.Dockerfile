FROM rustlang/rust:nightly-slim

RUN apt update && apt install -y gdbserver procps
