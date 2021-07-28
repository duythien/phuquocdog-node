FROM paritytech/ci-linux:production as builder
LABEL description="This is the build stage for Polkadot. Here we create the binary."

ARG PROFILE=release
WORKDIR /phuquocdog

COPY . /phuquocdog

RUN cargo build --$PROFILE

# ===== SECOND STAGE ======

FROM debian:buster-slim
LABEL description="This is the 2nd stage: a very small image where we copy the Polkadot binary."
ARG PROFILE=release
COPY --from=builder /phuquocdog/target/$PROFILE/phuquocdog /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /phuquocdog phuquocdog && \
	mkdir -p /phuquocdog/.local/share && \
	mkdir /data && \
	chown -R phuquocdog:phuquocdog /data && \
	ln -s /data /phuquocdog/.local/share/phuquocdog && \
	rm -rf /usr/bin /usr/sbin

USER phuquocdog
EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["/usr/local/bin/phuquocdog"]
