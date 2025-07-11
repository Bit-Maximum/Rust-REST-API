FROM rust:1.88

WORKDIR /app
COPY . .

RUN cargo install --path .

EXPOSE 3000

CMD ["app"]
