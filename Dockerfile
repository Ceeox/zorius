# source: https://www.lpalmieri.com/posts/fast-rust-docker-builds/
FROM rust:1.59.0-bullseye as planner
WORKDIR /zorius
RUN apt-get install -y openssl libssl-dev
RUN cargo install cargo-chef 
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json


FROM rust:1.59.0-bullseye as cacher
WORKDIR /zorius
RUN apt-get install -y openssl libssl-dev
RUN cargo install cargo-chef
COPY --from=planner /zorius/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
RUN cargo chef cook --recipe-path recipe.json


FROM rust:1.59.0-bullseye as builder
WORKDIR /zorius
COPY . .
RUN apt-get -y install openssl libssl-dev
# Copy over the cached dependencies
COPY --from=cacher /zorius/target /zorius/target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

RUN cargo build --release --bin zorius
RUN cargo test --bin zorius


FROM debian:bullseye-slim as runtime
WORKDIR /zorius
COPY --from=builder /zorius/target/release/zorius /zorius
COPY config/default.toml /zorius/config/default.toml

RUN mkdir -p static/avatar
RUN mkdir -p files

ENV DOMAIN=localhost
ENV SECRET_KEY=XtSQM8e7w33kR3XtHGWSjdm5gUhmtYEgPLPwkFyF8xLnSpuMr7QsbuSqG9jD4JZd8rbvKCK9BE8SmmVVFHgugzXQQ2e2pxBAZY3nZVE2TwCxAF76tkfcn6EdaLQYCcAJxVDnNJejndfXQM3t9qFw6apJftrsqUnz4vBnpe7AYZ5ZwAx36jdG6gaYzGxPNb4Dasa8SSSYryCTDLJREv4kK4J7nM5G5HxJhwPErrAzyzb26CPecbmVEECKDBTN9VET
ENV TOKEN_LIFETIME=86400

ENV WEB_IP=0.0.0.0
ENV WEB_PORT=8080
ENV WEB_ENABLE_SSL=false
ENV WEB_CERT_PATH=
ENV WEB_KEY_PATH=
ENV WEB_LOG_FORMAT=

ENV DB_HOST=localhost
ENV DB_PORT=5432
ENV DB_NAME=zorius
ENV DB_USERNAME=zorius
ENV DB_PASSWORD=zorius

EXPOSE ${WEB_PORT}
ENTRYPOINT ["/zorius/zorius"]
