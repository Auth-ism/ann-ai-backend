# Derleme aşaması
FROM rust:1.87.0-slim as builder
WORKDIR /usr/src/app

# Bağımlılıkları derle
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Kaynak kodu kopyala ve derle
COPY . .
RUN cargo build --release

# Çalışma aşaması
FROM debian:bookworm-slim
WORKDIR /app

# Çalışma zamanı bağımlılıkları
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Derlenmiş uygulamayı kopyala
COPY --from=builder /usr/src/app/target/release/ann-ai-backend /app/
COPY --from=builder /usr/src/app/migrations /app/migrations

ENV RUST_LOG=debug

CMD ["/app/ann-ai-backend"]