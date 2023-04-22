FROM rust:1.67

WORKDIR /yew_frontend_docker
COPY . .

RUN cargo install --path .

CMD ["yew_frontend_docker", "trunk serve"]