FROM rust:latest as builder
RUN cargo install wasm-pack
WORKDIR /usr/src/app
COPY ./app .
RUN wasm-pack build --target web


FROM httpd:2.4
COPY --from=builder /usr/src/app /usr/local/apache2/htdocs/