FROM rust:1.46.0-slim-buster

WORKDIR /app

RUN apt-get update \
    && apt-get install -y git nodejs npm \
    && npm install -g npm@latest \
    && npm install -g test262-harness


RUN mkdir -p /app && cd /app \
    && git clone https://github.com/tc39/test262.git --depth 1

COPY . /app

RUN cargo build --release

CMD tail -f /dev/null