FROM rust:1.85.0-slim

RUN apt update && apt install pkg-config libssl-dev -y

WORKDIR /app

CMD [ "cargo", "run", "--release" ]