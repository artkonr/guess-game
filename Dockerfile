FROM ekidd/rust-musl-builder AS build

ADD --chown=rust:rust . ./

RUN cargo build --release

WORKDIR .


FROM alpine

RUN mkdir app
COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release /app
ENTRYPOINT /app/guess-game
