FROM ubuntu:24.04

ARG RUST_TOOLCHAIN=stable
ARG SCCACHE_VERSION=v0.16.0
ARG CARGO_NEXTEST_VERSION=0.9.140
ARG CARGO_NEXTEST_X86_64_SHA256=4ee9aaa0d0171a985a5d0eb735b87355894c1c455972e9674fb9fdbd1387c9a3
ARG CARGO_NEXTEST_AARCH64_SHA256=8b3f4d4560b6b0f83774fecc6be07e47716dbad0eb0bb6c3890f478f4affe4b6
ARG CARGO_LLVM_COV_VERSION=0.8.7

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
      gh \
      jq \
      lld \
      openssh-client \
      pkg-config \
      python3 \
      tar \
      time \
      unzip \
      xz-utils \
      zstd \
      libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN arch="$(uname -m)" \
    && case "$arch" in \
      x86_64) awscli_arch="x86_64" ;; \
      aarch64|arm64) awscli_arch="aarch64" ;; \
      *) echo "unsupported awscli arch: $arch" >&2; exit 2 ;; \
    esac \
    && curl -fsSL "https://awscli.amazonaws.com/awscli-exe-linux-${awscli_arch}.zip" -o /tmp/awscliv2.zip \
    && unzip -q /tmp/awscliv2.zip -d /tmp \
    && /tmp/aws/install \
    && rm -rf /tmp/aws /tmp/awscliv2.zip

RUN curl -fsSL https://sh.rustup.rs -o /tmp/rustup-init.sh \
    && sh /tmp/rustup-init.sh -y --no-modify-path --profile minimal --default-toolchain "${RUST_TOOLCHAIN}" \
    && rm -f /tmp/rustup-init.sh \
    && rustup component add rustfmt clippy llvm-tools-preview

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

RUN arch="$(uname -m)" \
    && case "$arch" in \
      x86_64) nextest_arch="x86_64-unknown-linux-gnu"; nextest_sha256="$CARGO_NEXTEST_X86_64_SHA256" ;; \
      aarch64|arm64) nextest_arch="aarch64-unknown-linux-gnu"; nextest_sha256="$CARGO_NEXTEST_AARCH64_SHA256" ;; \
      *) echo "unsupported cargo-nextest arch: $arch" >&2; exit 2 ;; \
    esac \
    && nextest_archive="cargo-nextest-${CARGO_NEXTEST_VERSION}-${nextest_arch}.tar.gz" \
    && nextest_url="https://github.com/nextest-rs/nextest/releases/download/cargo-nextest-${CARGO_NEXTEST_VERSION}/${nextest_archive}" \
    && curl -fsSL "$nextest_url" -o "/tmp/${nextest_archive}" \
    && printf '%s  %s\n' "$nextest_sha256" "/tmp/${nextest_archive}" | sha256sum -c - \
    && mkdir -p /tmp/cargo-nextest \
    && tar -xzf "/tmp/${nextest_archive}" -C /tmp/cargo-nextest \
    && nextest_bin="$(find /tmp/cargo-nextest -type f -name cargo-nextest | head -n 1)" \
    && test -n "$nextest_bin" \
    && install -m 0755 "$nextest_bin" /usr/local/cargo/bin/cargo-nextest \
    && cargo nextest --version \
    && rm -rf /tmp/cargo-nextest "/tmp/${nextest_archive}"

RUN printf '%s\n' \
      "rustc=$(rustc --version)" \
      "cargo=$(cargo --version)" \
      "cargo-nextest=$(cargo nextest --version)" \
      "sccache=$(sccache --version)" \
      "lld=$(ld.lld --version | head -n 1)" \
      "aws=$(aws --version 2>&1)" \
      > /usr/local/share/adl-builder-toolchain.txt

RUN arch="$(uname -m)" \
    && case "$arch" in \
      x86_64) llvm_cov_arch="x86_64-unknown-linux-musl" ;; \
      aarch64|arm64) llvm_cov_arch="aarch64-unknown-linux-musl" ;; \
      *) echo "unsupported cargo-llvm-cov arch: $arch" >&2; exit 2 ;; \
    esac \
    && llvm_cov_url="https://github.com/taiki-e/cargo-llvm-cov/releases/download/v${CARGO_LLVM_COV_VERSION}/cargo-llvm-cov-${llvm_cov_arch}.tar.gz" \
    && curl -fsSL "$llvm_cov_url" -o /tmp/cargo-llvm-cov.tar.gz \
    && mkdir -p /tmp/cargo-llvm-cov \
    && tar -xzf /tmp/cargo-llvm-cov.tar.gz -C /tmp/cargo-llvm-cov \
    && llvm_cov_bin="$(find /tmp/cargo-llvm-cov -type f -name cargo-llvm-cov | head -n 1)" \
    && test -n "$llvm_cov_bin" \
    && install -m 0755 "$llvm_cov_bin" /usr/local/cargo/bin/cargo-llvm-cov \
    && cargo llvm-cov --version \
    && rm -rf /tmp/cargo-llvm-cov /tmp/cargo-llvm-cov.tar.gz

RUN gh --version

WORKDIR /workspace

ENTRYPOINT ["/bin/bash", "-lc"]
