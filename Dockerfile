# build the web app
FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev pkgconfig openssl-dev
WORKDIR /ourairports_api
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --bin web

# main image
FROM alpine:latest
COPY --from=builder /ourairports_api/target/x86_64-unknown-linux-musl/debug/web /usr/local/bin/ourairports_api
EXPOSE 8080
CMD ["ourairports_api"]