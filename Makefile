

default:
	@cargo clippy

build:
	@cargo build

build-release:
	@cargo build -q --release

build-examples:
	@cargo build -q --examples

test:
	@cargo test

doc:
	@cargo doc --no-deps

watch-doc:
	@cargo watch -x "doc --no-deps"