# ベースイメージとして公式のRustイメージを使用
FROM rust:latest

WORKDIR /usr/src/skill-check

RUN apt-get update && apt-get install -y \
  vim \
  && rm -rf /var/lib/apt/lists/*

# ソースコードをコンテナにコピー
COPY . .

# `cargo-watch`をインストール
RUN cargo install cargo-watch

# デフォルトのコマンドを設定
CMD ["cargo", "watch", "-x", "run"]