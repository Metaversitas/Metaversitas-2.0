FROM messense/rust-musl-cross:x86_64-musl as chef
ENV SQLX_OFFLINE=true
RUN cargo install cargo-chef
WORKDIR /backend

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /backend/recipe.json recipe.json

# Build cache & dependencies
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl


## Create a new minimal image
FROM ubuntu:latest as backend-app
COPY --from=builder /backend/target/x86_64-unknown-linux-musl/release/server /app/backend/server
COPY --from=builder /backend/certs/ /app/backend/certs
WORKDIR /app/backend/
CMD ["chmod", "+x", "/app/backend/server"]
CMD ["/app/backend/server"]