lint:
	cargo clippy

install:
	cargo build --release
	sudo rm -rf /usr/local/bin/beam
	sudo cp target/release/beam /usr/local/bin/beam