# build the web app
FROM rust:1 AS builder
WORKDIR /ourairports_api
COPY . .
RUN cargo build --release --bin web

# main image
FROM ubuntu:focal
RUN apt update
RUN apt -y install libssl1.1 ca-certificates
COPY --from=builder /ourairports_api/target/release/web /usr/local/bin/ourairports_api
#EXPOSE 8080
#CMD ["ourairports_api"]