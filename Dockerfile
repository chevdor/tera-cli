FROM rust:1.54.0-slim-buster as tera

# ******************************************************************************
# Install dependencies
# ******************************************************************************
RUN set -eux \
    && DEBIAN_FRONTEND=noninteractive apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends --no-install-suggests --yes \
        upx-ucl \
    && DEBIAN_FRONTEND=noninteractive apt-get autoremove --purge \
    && DEBIAN_FRONTEND=noninteractive apt-get autoclean \
    && DEBIAN_FRONTEND=noninteractive apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && echo ">>> FINISHED DEPENDENCIES INSTALL"

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
    && upx --best --lzma target/x86_64-unknown-linux-musl/release/tera \
    && mv target/x86_64-unknown-linux-musl/release/tera /usr/bin/ \
    && tera --version \
    && rm -rf target \
    && echo ">>> FINISHED COMPILING 'tera'"

# ------------------------------------------------------------------------------
# ------------------------------------------------------------------------------

FROM alpine

# ******************************************************************************
# Install compiled tera cli
# ******************************************************************************
COPY --from=tera /usr/bin/tera /usr/bin/tera

WORKDIR /opt

ENTRYPOINT ["/usr/bin/tera"]
