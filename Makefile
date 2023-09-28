

default:
	@cargo clippy -q

build:
	@cargo build

build-release:
	@cargo build --release

test:
	@cargo nextest r && cargo test --doc

doc:
	@cargo doc --no-deps

watch-doc:
	@RUSTDOCFLAGS="--cfg docsrs" cargo +nightly watch -x "doc --all-features"	