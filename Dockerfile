FROM rust:1.57.0

WORKDIR /url-mapper-rs

COPY . ./

ENV ENV=production

RUN cargo build --release

EXPOSE 3000

CMD ./target/release/url-mapper-rs