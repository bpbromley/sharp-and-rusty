FROM rust:1.58

WORKDIR /usr/src/rusty
COPY . .

RUN cargo install --path .

CMD ["rusty"]