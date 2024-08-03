FROM rust:1.80.0-alpine3.19 AS builder

ARG enable_cargo_mirror
USER root
WORKDIR /app
COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN apk add --no-cache musl-dev
COPY cargo.config /root/.cargo/config
RUN if [[ -z $enable_cargo_mirror ]]; then echo "" > ~/.cargo/config ;\
    else echo "Using Cargo mirror" ; fi
RUN cargo install --path .

FROM alpine:3.18
COPY --from=builder /usr/local/cargo/bin/eastwind-blogger /usr/bin/eastwind-blogger
WORKDIR /app
COPY log4rs.yml log4rs.yml
COPY static static
COPY templates templates
EXPOSE 8080
USER root

CMD ["eastwind-blogger", "-s"]
