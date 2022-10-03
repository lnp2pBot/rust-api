# Build Stage: lnp2pbot rust-api
# ------------------------------
FROM rust:alpine AS build

# Create App directory
WORKDIR /app

# Upddate
RUN apk update
RUN apk add musl-dev

# Copy source code, Cargo.toml, Cargo.lock, vendor to build directory
COPY . .

# vendor all dependencies locally, when compiling project
RUN cargo vendor

# build using linux-musl
RUN cargo install --path . --target=x86_64-unknown-linux-musl

# Execution stage: lnp2pbot rust-api
# ----------------------------------
FROM alpine
COPY --from=build /usr/local/cargo/bin/* /usr/local/bin/

# Run @lnp2pbot rust-api in port 8000
EXPOSE 8000
CMD ["/usr/local/bin/lnp2pbot-api"]
