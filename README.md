# WASM-NGINX

Repo for test asset load performance on wasm

## Commands

### Build the web-assets container

```shell
docker build -t nginx-gzip-brotli web-assets
```

### Build the wasm application container

```shell
docker build -t wasm-nginx  .
```

### Run both containers

```shell
docker compose up -d
```
