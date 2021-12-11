FROM ubuntu as builder
RUN apt-get update
RUN apt-get install -y git && apt-get install -y curl
RUN git clone https://github.com/phuquocdog/node
RUN cd node && \
	curl -O -L https://raw.githubusercontent.com/phuquocdog/node/master/node/res/phuquocdog.json  && \
    apt-get install -y build-essential && \
    apt-get install -y clang && \
    apt-get install -y jq && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
        export PATH="$PATH:$HOME/.cargo/bin" && \
        rustup toolchain install nightly-2021-06-28 && \
        rustup target add wasm32-unknown-unknown --toolchain nightly-2021-06-28 && \
        rustup default stable && \
        # cargo update -p sp-std --precise 88c64e06471cc12aa9b25290f24d5566bcb5dd82 && \
        cargo +nightly-2021-06-28 build --release

# /\-Build Stage | Final Stage-\/

FROM docker.io/library/ubuntu:20.04
COPY --from=builder /node/target/release/phuquocdog-node /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /phuquocdog-node phuquocdog-node && \
        mkdir -p /phuquocdog-node/.local/share && \
        mkdir /data && \
        mkdir /node && \
        chown -R phuquocdog-node:phuquocdog-node /data && \
        chown -R phuquocdog-node:phuquocdog-node /node && \
        ln -s /data /phuquocdog-node/.local/share/phuquocdog-node && \
        rm -rf /usr/bin /usr/sbin

COPY --from=builder /node/phuquocdog.json /node/phuquocdog.json

USER phuquocdog-node
EXPOSE 30333 9933 9944
VOLUME ["/data"]

EXPOSE 30333 9933 9944

ENTRYPOINT ["/usr/local/bin/phuquocdog-node"]

# You should be able to run a validator using this docker image in a bash environmment with the following command:
