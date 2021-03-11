FROM ekidd/rust-musl-builder:latest AS builder

# Add our source code.
ADD --chown=rust:rust Cargo.toml Cargo.lock ./
ADD --chown=rust:rust src ./src

# Build our application.
RUN cargo build --release

# Now, we need to build our _real_ Docker container, copying in `wban-hot-wallet-monitor`.
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/wban-hot-wallet-monitor \
    /usr/local/bin/
CMD /usr/local/bin/wban-hot-wallet-monitor