FROM rust:1.77 as builder

WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /app/target/release/http-echo /usr/local/bin/http-echo

ENV LISTEN="0.0.0.0:5678"
ENV TEXT="hello-world"

CMD ["/usr/local/bin/http-echo", "--listen", "$LISTEN", "--text", "$TEXT"]
