lint:
	cargo clippy

local-install:
	cargo build --release
	sudo rm -rf /usr/local/bin/beam
	sudo cp target/release/beam /usr/local/bin/beam