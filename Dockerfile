FROM rust:1.84.1

WORKDIR /app
COPY . .

# Mise à jour de Cargo vers la dernière version stable
RUN rustup update stable

RUN cargo build

RUN useradd -m appuser

USER appuser
EXPOSE 8080
CMD ["/app/target/debug/wik_dps_tp01"]
