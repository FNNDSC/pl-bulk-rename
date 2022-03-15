FROM rust:1.59-slim-bullseye as builder
ARG CARGO_TERM_COLOR=always
WORKDIR /usr/local/src/bulkrename
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /usr/local/src/bulkrename/target/release/bulkrename /usr/local/bin/bulkrename
COPY docker-entrypoint.sh chris_plugin_info.json /
CMD ["bulkrename"]
