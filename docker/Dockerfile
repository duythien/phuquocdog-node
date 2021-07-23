FROM paritytech/ci-linux:production as builder
LABEL description="This is the build stage for Polkadot. Here we create the binary."

ARG PROFILE=release
WORKDIR /phuquocdoge

COPY . /phuquocdoge

RUN cargo build --$PROFILE

# ===== SECOND STAGE ======

FROM debian:buster-slim
LABEL description="This is the 2nd stage: a very small image where we copy the Polkadot binary."
ARG PROFILE=release
COPY --from=builder /phuquocdoge/target/$PROFILE/phuquocdoge /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /phuquocdoge phuquocdoge && \
	mkdir -p /phuquocdoge/.local/share && \
	mkdir /data && \
	chown -R phuquocdoge:phuquocdoge /data && \
	ln -s /data /phuquocdoge/.local/share/phuquocdoge && \
	rm -rf /usr/bin /usr/sbin

USER phuquocdoge
EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["/usr/local/bin/phuquocdoge"]
