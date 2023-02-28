FROM busybox:latest

COPY ./target/x86_64-unknown-linux-musl/release/uv-api-users /bin/uv-api-users
COPY ./Rocket.toml /root

WORKDIR /root

CMD [ "uv-api-users" ]
