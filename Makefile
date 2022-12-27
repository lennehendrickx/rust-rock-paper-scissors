.PHONY: build

build:
	#docker build . --target builder
	docker build . -t rust-rock-paper-scissors

run:
	docker run -d --name rust-rock-paper-scissors -p 8080:80 rust-rock-paper-scissors