# source: https://www.lpalmieri.com/posts/fast-rust-docker-builds/
FROM rust:alpine as planner
WORKDIR /zorius
# Install deps
RUN apk add --no-cache musl-dev

RUN cargo install cargo-chef 
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json


FROM rust:alpine as cacher
WORKDIR /zorius
# Install deps
RUN apk add --no-cache musl-dev openssl-dev

RUN cargo install cargo-chef
COPY --from=planner /zorius/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
RUN cargo chef cook --recipe-path recipe.json


FROM rust:alpine as builder
WORKDIR /zorius
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /zorius/target /zorius/target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
# Install deps
RUN apk add --no-cache musl-dev openssl-dev

RUN cargo build --release --bin zorius
RUN cargo test --bin zorius


FROM alpine as runtime
WORKDIR /zorius
COPY --from=builder /zorius/target/release/zorius /zorius
COPY config/default.toml /zorius/config/default.toml
RUN apk add --no-cache openssl

ENV DEBUG=false
ENV SECRET_KEY=
ENV DOMAIN=localhost
ENV TOKEN_LIFETIME=86400
ENV WEB_IP=127.0.0.1
ENV WEB_PORT=8080
ENV WEB_ENABLE_SSL=false
ENV WEB_CERT_PATH=
ENV WEB_KEY_PATH=
ENV WEB_LOG_FORMAT=
ENV DB_SERVER_DOMAIN=localhost
ENV DB_USERNAME=
ENV DB_PASSWORD=
ENV DB_APP_NAME=zorius
ENV DB_NAME=zorius

EXPOSE ${WEB_PORT}
ENTRYPOINT ["/zorius/zorius"]
