FROM rust:latest as build

WORKDIR /usr/src/poster
COPY . .

RUN cargo build --release

FROM debian:bullseye

COPY --from=build /usr/src/poster/target/release/poster /usr/local/bin/poster
COPY --from=build /usr/src/poster/static /usr/local/bin/static
COPY --from=build /usr/src/poster/.env /usr/local/bin/.env

WORKDIR /usr/local/bin
CMD ["poster"]