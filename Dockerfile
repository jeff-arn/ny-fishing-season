FROM rust:1.43 as builder

RUN USER=root cargo new --bin ny-fishing-season
WORKDIR /rust-docker-web

# puts the app dependencies on its own layer in the image
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/ny-fishing-season*
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
  && apt-get install -y ca-certificates tzdata \
  && rm -rf /var/lib/apt/lists/*

EXPOSE 4000

ENV TZ=Etc/UTC \
  APP_USER=appuser

RUN groupadd $APP_USER \
  && useradd -g $APP_USER $APP_USER \
  && mkdir -p ${APP}

COPY --from=builder /ny-fishing-season/target/release/ny-fishing-season ${APP}/ny-fishing-season

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./ny-fishing-season"]