FROM rust:1.71.0-slim as builder

WORKDIR /usr/src

# Create blank project. Remember to change the project's name
RUN USER=root cargo new axum_postgres_docker

# We want dependencies cached, so copy those (Cargo.toml, Cargo.lock) first
## !!! PLEASE NOTE: I did not copy 'Cargo.lock' because this particular project is a workspace member without its own 'Cargo.lock'
COPY Cargo.toml /usr/src/axum_postgres_docker/

# Set the working directory
WORKDIR /usr/src/axum_postgres_docker

# Install 'musl-tools' to enable successful image build
RUN apt-get -y update 
RUN apt-get -y upgrade
RUN apt-get -y install musl-tools

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl

# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-musl --release

# Now copy in the rest of the sources
COPY src /usr/src/axum_postgres_docker/src/

# Copy your migrations and file 'build.rs' too
COPY migrations /usr/src/axum_postgres_docker/migrations
COPY build.rs /usr/src/axum_postgres_docker

## Touch main.rs to prevent cached release build
RUN touch /usr/src/axum_postgres_docker/src/main.rs

# This is the actual application build.
RUN cargo build --target x86_64-unknown-linux-musl --release

################
##### Runtime
FROM alpine:3.16.0 AS runtime

# Copy application binary from builder image
COPY --from=builder /usr/src/axum_postgres_docker/target/x86_64-unknown-linux-musl/release/axum_postgres_docker /usr/local/bin

EXPOSE 8090

# Run the application via 'axum_postgres_docker.exe'
CMD ["/usr/local/bin/axum_postgres_docker"]