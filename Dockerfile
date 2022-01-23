# TODO: redo this dockerfile
# build the web app and docs
FROM rust:1 AS builder
WORKDIR /ourairports_api
COPY . .

# build the web app
RUN cargo build --release -p ourairports_api

# build the rust docs
RUN cargo doc

# build the API docs
WORKDIR /ourairports_api/docs
RUN cargo install mdbook
RUN mdbook build


# main image
FROM ubuntu:focal
RUN apt update
RUN apt -y install libssl1.1 ca-certificates
WORKDIR /ourairports_api
COPY --from=builder /ourairports_api/target/release/ourairports_api .
COPY --from=builder /ourairports_api/static ./static
COPY --from=builder /ourairports_api/target/doc ./static/rust-docs
COPY --from=builder /ourairports_api/docs/book ./static/docs