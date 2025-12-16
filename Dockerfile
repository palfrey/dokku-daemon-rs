FROM alpine:3.22 as builder
ARG HOST=x86_64-unknown-linux-musl

RUN apk add --no-cache rustup gcc file musl-dev
RUN rustup-init -y --default-host $HOST --profile minimal
ENV PATH=$PATH:/root/.cargo/bin

WORKDIR /app
ADD . ./
RUN cargo build --release --target=$HOST
RUN file ./target/$HOST/release/dokku-daemon-rs
RUN ls -lh ./target/$HOST/release/dokku-daemon-rs

FROM scratch
ARG HOST=x86_64-unknown-linux-musl
COPY --from=builder /app/target/$HOST/release/dokku-daemon-rs /dokku-daemon-rs
ENTRYPOINT ["/dokku-daemon-rs"]
