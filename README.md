# openfaas-rust-axum-template

This is a Rust template for [OpenFaaS](https://www.openfaas.com/) using the [Axum](https://github.com/tokio-rs/axum) framework. It is an async(Tokio) alternative to the [rust-http-template](https://github.com/openfaas-incubator/rust-http-template/tree/master) maintained by the OpenFaaS community. This template depends on Docker BuildKit to cache dependencies and speed up build times.

## Usage

1. Pull the template

```bash
$ faas template pull https://github.com/shedrachokonofua/openfaas-rust-axum-template

$ faas new --list
Languages available as templates:
- rust-axum
```

2. Create a new function

```bash
$ faas new --lang rust-axum <fn-name>
```

3. Build and deploy

```bash
DOCKER_BUILDKIT=1 faas-cli up -f <fn-name>.yml
```
