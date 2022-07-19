FROM rust:1.61

WORKDIR /usr/src/ethaccinfo.g.khassanov.xyz/backend
COPY . .

RUN cargo install --path .

EXPOSE 8100

CMD ["backend"]
