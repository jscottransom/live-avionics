FROM ekidd/rust-musl-builder as builder

RUN USER=root cargo new --bin live-flights
WORKDIR ./live-flights
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./configuration.yaml ./configuration.yaml
COPY .env .env
RUN sudo chmod -R 777 .
RUN cargo update
RUN cargo build --release
RUN rm src/*.rs

ADD . ./
RUN sudo chmod -R 777 .
RUN rm ./target/x86_64-unknown-linux-musl/release/deps/liveflights*
RUN sudo chmod -R 777 .
RUN cargo update
RUN cargo build --release

FROM alpine:latest
ARG APP=/usr/src/app

# EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER 

RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/lib/cache/apk/*

COPY --from=builder /home/rust/src/live-flights/target/x86_64-unknown-linux-musl/release/liveflights ${APP}/live-flights

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./live-flights"]