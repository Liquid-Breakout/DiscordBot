FROM rust:slim-buster as build

WORKDIR /app

COPY . /app
USER root

RUN apt-get update
RUN apt-get install libssl-dev pkg-config git -y
RUN cargo build --release

# Copy the binary into a new container for a smaller docker image
FROM debian:buster-slim

WORKDIR /etc/liquid_breakout_discordbot

RUN apt-get update \
    && apt-get install -y ca-certificates libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/liquid_breakout_discordbot ./
USER root
ENTRYPOINT ["./liquid_breakout_discordbot"]