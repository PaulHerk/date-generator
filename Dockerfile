FROM rust:1.68.2

WORKDIR /usr/src/apd
COPY . .

RUN cargo install --path .

WORKDIR /output
ENTRYPOINT ["apd"]