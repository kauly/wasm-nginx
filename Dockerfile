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

FROM nginx:1.23.3-alpine
WORKDIR /root/

RUN apk add --update --no-cache git pcre-dev openssl-dev zlib-dev linux-headers build-base \
   && wget http://nginx.org/download/nginx-1.23.3.tar.gz \
   && tar zxf nginx-1.23.3.tar.gz \
   && git clone https://github.com/google/ngx_brotli.git \
   && cd ngx_brotli \
   && git submodule update --init --recursive \
   && cd ../nginx-1.23.3 \
   && ./configure --add-dynamic-module=../ngx_brotli --with-compat --with-file-aio --with-threads --with-http_addition_module --with-http_auth_request_module --with-http_dav_module --with-http_flv_module --with-http_gunzip_module --with-http_gzip_static_module --with-http_mp4_module --with-http_random_index_module --with-http_realip_module --with-http_secure_link_module --with-http_slice_module --with-http_ssl_module --with-http_stub_status_module --with-http_sub_module --with-http_v2_module --with-mail --with-mail_ssl_module --with-stream --with-stream_realip_module --with-stream_ssl_module --with-stream_ssl_preread_module --with-cc-opt='-Os -fomit-frame-pointer -g' --with-ld-opt=-Wl,--as-needed,-O1,--sort-common \
   && make modules

COPY nginx.conf /etc/nginx/nginx.conf
COPY --from=build /shell-project/dist /usr/share/nginx/html

