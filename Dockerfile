FROM rust:1.67.1-alpine

ARG enable_cargo_mirror
USER root
WORKDIR /app
COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY log4rs.yml log4rs.yml
COPY static static
COPY templates templates
RUN apk add --no-cache musl-dev
COPY cargo.config /root/.cargo/config
RUN if [[ -z $enable_cargo_mirror ]]; then echo "" > ~/.cargo/config ;\
    else echo "Using Cargo mirror" ; fi
RUN cargo install --path .
RUN rm -rf ./src ./target Cargo.lock Cargo.toml /usr/local/cargo/registry/
RUN rustup uninstall 1.67.1-x86_64-unknown-linux-musl
EXPOSE 8080

CMD ["eastwind-blogger", "-s"]
