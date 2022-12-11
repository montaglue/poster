FROM rust:latest as build

WORKDIR /usr/src/poster
COPY . .

RUN cargo build --release

FROM debian:bullseye

COPY --from=build /usr/src/poster/target/release/poster /usr/local/bin/poster
COPY --from=build /usr/src/poster/static /usr/local/bin/static

WORKDIR /usr/local/bin
CMD ["poster"]