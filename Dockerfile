FROM rust:1.62 as builder
WORKDIR /src
COPY ./build/config /usr/local/cargo/
COPY . .
RUN cargo install --path .

FROM alpine:latest
WORKDIR /data
COPY --from=builder /usr/local/cargo/bin/user-center /usr/local/bin/user-center
CMD ["user-center"]
EXPOSE 8000
