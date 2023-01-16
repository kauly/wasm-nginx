FROM rust:1.66.1-slim-buster as build

RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config 

RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli trunk
RUN cargo new shell-project
WORKDIR /shell-project

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./assets ./assets
COPY ./src ./src
COPY ./index.html ./index.html

RUN trunk build --release

FROM nginx:alpine-slim

COPY --from=build /shell-project/dist /usr/share/nginx/html

