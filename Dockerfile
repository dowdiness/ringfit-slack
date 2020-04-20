FROM rust:latest

# # キャッシュ
# RUN cargo install cargo-build-deps

# リクエストを送るURL
ARG SLACK_WEBHOOK_URL

RUN mkdir /var/app
WORKDIR /var/app

ENV USER root

RUN cargo new temp

WORKDIR /var/app/temp

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

RUN rm -rf ./src

# Rustのコードのみの変更時はここまでキャッシュが効く
COPY . .

CMD ["cargo", "run", "--release"]
