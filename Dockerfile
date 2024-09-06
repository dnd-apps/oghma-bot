# ------------------- #
# -- Oghma Planner -- #
# ------------------- #
FROM lukemathwalker/cargo-chef:latest-rust-1.63 as planner
WORKDIR /data/oghma
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ------------------- #
# -- Oghma Cacher  -- #
# ------------------- #
FROM lukemathwalker/cargo-chef:latest-rust-1.63 as cacher
WORKDIR /data/oghma
COPY --from=planner /data/oghma/recipe.json recipe.json
RUN rustup target add x86_64-unknown-linux-gnu
RUN cargo chef cook --release --target x86_64-unknown-linux-gnu --recipe-path recipe.json


# ------------------- #
# -- Oghma Builder -- #
# ------------------- #
FROM rust:1.81 as builder
WORKDIR /data/oghma
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /data/oghma/target target
COPY --from=cacher /usr/local/cargo/registry /usr/local/cargo/
RUN cargo build --release

# ------------------- #
# -- Oghma Runtime -- #
# ------------------- #
FROM debian:11-slim as runtime
WORKDIR /apps
COPY --from=builder /data/oghma/target/x86_64-unknown-linux-gnu/release/oghma-bot /data/oghma/target/release/oghma-bot ./
ENTRYPOINT ["/apps/oghma-bot"]
#CMD ["--version"]




#FROM rust:1.63.0-bullseye AS builder
#
### Install target platform (Cross-Compilation) --> Needed for Alpine
#RUN rustup target add x86_64-unknown-linux-gnu
#
#WORKDIR /app
#
#COPY . .
#
## This is a dummy build to get the dependencies cached.
#RUN cargo build --target x86_64-unknown-linux-gnu --release
#
#FROM debian:bullseye
#
#WORKDIR /bot
#
#COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/oghma-bot /bot/oghma-bot
#
#ENTRYPOINT [ "/bot/oghma-bot" ]
