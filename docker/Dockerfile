FROM docker.io/paritytech/ci-linux:production as builder
RUN git clone https://github.com/phuquocdog/node && cd node && \
    cargo build --release --features=runtime-benchmarks

# /\-Build Stage | Final Stage-\/

FROM docker.io/library/ubuntu:20.04
COPY --from=builder /builds/node/target/release/phuquocdog-node /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /phuquocdog-node phuquocdog-node && \
        mkdir -p /phuquocdog-node/.local/share && \
        mkdir /data && \
        mkdir /node && \
        chown -R phuquocdog-node:phuquocdog-node /data && \
        chown -R phuquocdog-node:phuquocdog-node /node && \
        ln -s /data /phuquocdog-node/.local/share/phuquocdog-node && \
        rm -rf /usr/bin /usr/sbin

USER phuquocdog-node
EXPOSE 30333 9933 9944
VOLUME ["/data"]

EXPOSE 30333 9933 9944

ENTRYPOINT ["/usr/local/bin/phuquocdog-node"]

# You should be able to run a validator using this docker image in a bash environmment with the following command:
