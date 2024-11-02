
build:
	cargo build --target aarch64-apple-ios-sim --example simple

bundle: build
	cp ./target/aarch64-apple-ios-sim/debug/examples/simple ./RustWrapper.app/

install: bundle
	xcrun simctl install booted ./RustWrapper.app/

run: install
	SIMCTL_CHILD_RUST_BACKTRACE=full SIMCTL_CHILD_RUST_LOG=trace xcrun simctl launch --console --terminate-running-process booted RustWrapper

watch:
	cargo watch -s 'make run' -w ./src -w ./Cargo.toml -w ./examples/

build-macabi:
	cargo +nightly build --target aarch64-apple-ios-macabi -Zbuild-std --example simple

bundle-macabi: build-macabi
	cp ./target/aarch64-apple-ios-macabi/debug/examples/simple ./RustWrapper.app/

run-macabi: bundle-macabi
	open ./RustWrapper.app/

watch-macabi:
	cargo watch -s 'make run-macabi' -w ./src -w ./Cargo.toml -w ./examples/
