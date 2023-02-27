FROM busybox:latest

COPY ./target/x86_64-unknown-linux-musl/release/q-api-users /bin/q-api-users
COPY ./Rocket.toml /root

WORKDIR /root

CMD [ "q-api-users" ]
