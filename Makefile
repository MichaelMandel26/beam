lint:
	cargo clippy

symlinc:
	cargo build --release
	sudo cp target/release/beam /usr/local/bin/beam