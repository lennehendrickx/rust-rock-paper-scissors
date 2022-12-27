.PHONY: build, run, local-lint, local-build

REPOSITORY=rust-rock-paper-scissors
VERSION=latest

build:
	docker build --target web-server -t ${REPOSITORY}/web-server:${VERSION} .

run:
	docker run -d --name rust-rock-paper-scissors -p 8080:80 ${REPOSITORY}/web-server:${VERSION}

local-build:
	cd app && cargo clippy && wasm-pack build --target web
