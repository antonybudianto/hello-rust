build-native:
	rustc src/main.rs

build:
	cargo build

build-prod:
	cargo build --release

run: # similar to ./target/debug/hello_rust
	cargo run

run-prod: # similar to ./target/release/hello_rust
	cargo run --release
