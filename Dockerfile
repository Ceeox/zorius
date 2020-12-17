FROM rust:1.48-alpine

WORKDIR /usr/src/zorius
COPY . .

RUN cargo install --path .
RUN rm -rf ./target/

CMD ["zorius"]