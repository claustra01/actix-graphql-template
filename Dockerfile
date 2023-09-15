FROM rust:slim-buster
RUN apt update && apt upgrade && apt install -y libpq-dev
WORKDIR /opt

COPY . .
EXPOSE 8080

RUN cargo build --release
ENTRYPOINT ["./target/release/hackz-tyranno"]