FROM rust:1.67 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/wodemao-server /usr/local/bin/wodemao-server
EXPOSE 3000
CMD ["wodemao-server"]