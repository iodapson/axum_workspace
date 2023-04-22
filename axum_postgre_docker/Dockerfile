FROM rust:1.67

WORKDIR /axum_postgre_docker
COPY . .

RUN cargo install --path .

CMD ["axum_postgre_docker", "cargo watch -x run"]