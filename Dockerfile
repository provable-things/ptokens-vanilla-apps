ARG os_version=1.46.0
ARG core_version=latest

FROM provable/ptokens-core:$core_version
FROM rust:$os_version-slim-buster

ENV HOME /home/provable
ENV FOLDER_VANILLA $HOME/vanilla

WORKDIR $HOME

RUN groupadd provable && \
    useradd -m -g provable provable && \
    mkdir -p $FOLDER_VANILLA && \
    chown -R provable:provable $FOLDER_VANILLA && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
        clang && \
    rm -rf /var/lib/apt/lists/* && \
    apt-get autoremove -y && \
    apt-get clean

COPY --chown=provable:provable --from=0 /root/core $HOME/ptokens-core-private
COPY --chown=provable:provable src $FOLDER_VANILLA/src
COPY --chown=provable:provable Cargo.* $FOLDER_VANILLA/

USER provable

WORKDIR $FOLDER_VANILLA

RUN cargo build --release

ENTRYPOINT ["./target/release/vanilla"]