FROM rust:1.59-slim-bullseye as builder
WORKDIR /usr/local/src/bulkrename
COPY . .
RUN cargo build --release
RUN strip target/release/bulkrename

FROM debian:bullseye-slim
COPY --from=builder /usr/local/src/bulkrename/target/release/bulkrename /usr/local/bin/bulkrename
COPY /docker-entrypoint.sh /docker-entrypoint.sh
COPY chris_plugin_info.json /chris_plugin_info.json
CMD ["bulkrename"]
