FROM ubuntu:24.04

ARG RUST_TOOLCHAIN=stable
ARG SCCACHE_VERSION=v0.16.0

ENV DEBIAN_FRONTEND=noninteractive
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH=/usr/local/cargo/bin:/usr/local/bin:/usr/bin:/bin
ENV CARGO_INCREMENTAL=0
ENV RUSTC_WRAPPER=sccache
ENV SCCACHE_CACHE_SIZE=20G
ENV RUSTFLAGS="-C link-arg=-fuse-ld=lld --remap-path-prefix=/workspace=/workspace --remap-path-prefix=/root=/home"

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends \
      bash \
      build-essential \
      ca-certificates \
      clang \
      curl \
      git \
      jq \
      lld \
      openssh-client \
      pkg-config \
      python3 \
      tar \
      unzip \
      xz-utils \
      zstd \
      libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN curl -fsSL https://sh.rustup.rs -o /tmp/rustup-init.sh \
    && sh /tmp/rustup-init.sh -y --no-modify-path --profile minimal --default-toolchain "${RUST_TOOLCHAIN}" \
    && rm -f /tmp/rustup-init.sh \
    && rustup component add rustfmt clippy

RUN arch="$(uname -m)" \
    && case "$arch" in \
      x86_64) sccache_arch="x86_64-unknown-linux-musl" ;; \
      aarch64|arm64) sccache_arch="aarch64-unknown-linux-musl" ;; \
      *) echo "unsupported sccache arch: $arch" >&2; exit 2 ;; \
    esac \
    && sccache_archive="sccache-${SCCACHE_VERSION}-${sccache_arch}.tar.gz" \
    && curl -fsSL "https://github.com/mozilla/sccache/releases/download/${SCCACHE_VERSION}/${sccache_archive}" -o "/tmp/${sccache_archive}" \
    && tar -xzf "/tmp/${sccache_archive}" -C /tmp \
    && install -m 0755 "/tmp/sccache-${SCCACHE_VERSION}-${sccache_arch}/sccache" /usr/local/bin/sccache \
    && rm -rf "/tmp/${sccache_archive}" "/tmp/sccache-${SCCACHE_VERSION}-${sccache_arch}"

WORKDIR /workspace

ENTRYPOINT ["/bin/bash", "-lc"]
