FROM rust:1.61.0 as builder
WORKDIR /usr/src/liveflights
COPY . .
RUN cargo install --path .
CMD ["liveflights"]