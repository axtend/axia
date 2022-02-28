# This is the build stage for Axia. Here we create the binary in a temporary image.
FROM docker.io/axiatech/ci-linux:production as builder

WORKDIR /polkadot
COPY . /polkadot

RUN cargo build --locked --release

# This is the 2nd stage: a very small image where we copy the Axia binary."
FROM docker.io/library/ubuntu:20.04

LABEL description="Multistage Docker image for Axia: a platform for web3" \
	io.axia.image.type="builder" \
	io.axia.image.authors="chevdor@gmail.com, devops-team@axia.io" \
	io.axia.image.vendor="Axia Technologies" \
	io.axia.image.description="Axia: a platform for web3" \
	io.axia.image.source="https://github.com/axiatech/polkadot/blob/${VCS_REF}/scripts/dockerfiles/polkadot/polkadot_builder.Dockerfile" \
	io.axia.image.documentation="https://github.com/axiatech/polkadot/"

COPY --from=builder /polkadot/target/release/polkadot /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /polkadot polkadot && \
	mkdir -p /data /polkadot/.local/share && \
	chown -R polkadot:polkadot /data && \
	ln -s /data /polkadot/.local/share/polkadot && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/polkadot --version

USER polkadot

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/polkadot"]
