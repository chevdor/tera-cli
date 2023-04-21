FROM rust:1.69.0-slim-buster as builder

ARG DEBIAN_FRONTEND=noninteractive

# ******************************************************************************
# Copy source code into container
# ******************************************************************************
COPY . /opt/

# ******************************************************************************
# Compile tera cli application from source
# ******************************************************************************
RUN set -eux \
    && cd /opt/ \
    && rustup target add x86_64-unknown-linux-musl \
    && cargo build --release --target x86_64-unknown-linux-musl \
    && mv target/x86_64-unknown-linux-musl/release/tera /usr/bin/ \
    && tera --version \
    && rm -rf target \
    && echo ">>> FINISHED COMPILING 'tera-cli'"

# ------------------------------------------------------------------------------

FROM alpine

# ******************************************************************************
# Install compiled tera cli
# ******************************************************************************
COPY --from=builder /usr/bin/tera /usr/bin/tera

USER guest

WORKDIR /opt

ENTRYPOINT ["/usr/bin/tera"]
