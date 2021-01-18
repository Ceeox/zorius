# source: https://www.lpalmieri.com/posts/fast-rust-docker-builds/
FROM rust as planner
WORKDIR zorius
RUN cargo install cargo-chef 
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust as cacher
WORKDIR zorius
RUN cargo install cargo-chef
COPY --from=planner /zorius/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as builder
WORKDIR zorius
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /zorius/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin zorius

FROM rust as runtime
WORKDIR zorius
COPY --from=builder /zorius/target/release/zorius /usr/local/bin
ENTRYPOINT ["./usr/local/bin/zorius"]