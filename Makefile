
build:
	cargo build --target aarch64-apple-ios-sim --example simple

bundle: build
	cp ./target/aarch64-apple-ios-sim/debug/examples/simple ./RustWrapper.app/

install: bundle
	xcrun simctl install booted ./RustWrapper.app/

debug: install
	SIMCTL_CHILD_RUST_BACKTRACE=full SIMCTL_CHILD_RUST_LOG=trace xcrun simctl launch --wait-for-debugger --console --terminate-running-process booted RustWrapper

run: install
	SIMCTL_CHILD_RUST_BACKTRACE=full SIMCTL_CHILD_RUST_LOG=trace xcrun simctl launch --console --terminate-running-process booted RustWrapper

screenshot: install
	SIMCTL_CHILD_RUST_BACKTRACE=full SIMCTL_CHILD_RUST_LOG=trace xcrun simctl launch --stdout=$(PWD)/stdout.txt --stderr=$(PWD)/stderr.txt --terminate-running-process booted RustWrapper
	sleep 2
	xcrun simctl io booted screenshot screenshot.png

record: install
	SIMCTL_CHILD_RUST_BACKTRACE=full SIMCTL_CHILD_RUST_LOG=trace xcrun simctl launch --stdout=$(PWD)/stdout.txt --stderr=$(PWD)/stderr.txt --terminate-running-process booted RustWrapper --record
	xcrun simctl io booted recordVideo -f record.mp4 &
	sleep 2
	ps | grep 'simctl io booted recordVideo' | grep -v grep | awk '{print $$1}' | xargs kill -s SIGINT

gh-summary:
	echo "## APP STDOUT" > Summary.md
	echo \`\`\` >> Summary.md
	cat stdout.txt >> Summary.md
	echo \`\`\` >> Summary.md
	echo "## APP STDERR" >> Summary.md
	echo \`\`\` >> Summary.md
	cat stderr.txt >> Summary.md
	echo \`\`\` >> Summary.md
	echo "## SCREENSHOT" >> Summary.md
	echo "![](data:image/png;base64,$(shell base64 -i screenshot.png))" >> Summary.md

watch:
	cargo watch -s 'make run' -w ./src -w ./Cargo.toml -w ./examples/

build-macabi:
	cargo build --target aarch64-apple-ios-macabi -Zbuild-std --example simple

bundle-macabi: build-macabi
	cp ./target/aarch64-apple-ios-macabi/debug/examples/simple ./RustWrapper.app/

run-macabi: bundle-macabi
	open ./RustWrapper.app/

watch-macabi:
	cargo watch -s 'make run-macabi' -w ./src -w ./Cargo.toml -w ./examples/
