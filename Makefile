HUB ?= quay.io/isanchez
IMAGE ?= ossm-example-headers-extension
VERSION ?= 1.0.0

build: wasm

wasm:
	cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/ossm_example_headers_extension.wasm ./plugin.wasm

.PHONY: clean
clean:
	rm plugin.wasm || true
	rm -r build || true

.PHONY: container
image: clean build
	mkdir build
	cp container/manifest.yaml build/
	cp plugin.wasm build/
	cd build && podman build -t ${HUB}/${IMAGE}:${VERSION} . -f ../container/Dockerfile

image.push: image
	podman push ${HUB}/${IMAGE}:${VERSION}
