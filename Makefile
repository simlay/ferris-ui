EMULATOR='iPhone 16'
DEVICE_ID=YOUR_DEVICE_ID
TEAM_ID=YOUR_TEAM_ID

build:
	cargo build --target aarch64-apple-ios-sim --example simple

bundle: build
	cp ./target/aarch64-apple-ios-sim/debug/examples/simple ./RustWrapper.app/

install: bundle
	xcrun simctl install $(EMULATOR) ./RustWrapper.app/

debug: install
	SIMCTL_CHILD_RUST_BACKTRACE=full SIMCTL_CHILD_RUST_LOG=trace xcrun simctl launch --wait-for-debugger --console --terminate-running-process $(EMULATOR) RustWrapper

run: install
	SIMCTL_CHILD_RUST_BACKTRACE=full SIMCTL_CHILD_RUST_LOG=trace xcrun simctl launch --console --terminate-running-process $(EMULATOR) com.simlay.net.Dinghy

screenshot: install
	SIMCTL_CHILD_RUST_BACKTRACE=full SIMCTL_CHILD_RUST_LOG=trace xcrun simctl launch --stdout=$(PWD)/stdout.txt --stderr=$(PWD)/stderr.txt --terminate-running-process $(EMULATOR) RustWrapper
	sleep 2
	xcrun simctl io $(EMULATOR) screenshot screenshot.png
	sips -Z 1278 screenshot.png

record: install
	SIMCTL_CHILD_RUST_BACKTRACE=full SIMCTL_CHILD_RUST_LOG=trace xcrun simctl launch --stdout=$(PWD)/stdout.txt --stderr=$(PWD)/stderr.txt --terminate-running-process $(EMULATOR) RustWrapper --record
	xcrun simctl io $(EMULATOR) recordVideo -f record.mp4 &
	sleep 2
	ps | grep 'simctl io $(EMULATOR)  recordVideo' | grep -v grep | awk '{print $$1}' | xargs kill -s SIGINT

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
	echo "![Screenshot](${SCREENSHOT_URL})" >> Summary.md

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

device-build:
	cargo build --target aarch64-apple-ios --example simple

device-bundle: device-build
	cp ./target/aarch64-apple-ios/debug/examples/simple ./RustWrapper.app/

device-clear-entitlements:
	/usr/libexec/PlistBuddy -x -c "Delete :application-identifier" ./RustWrapper.app/entitlements.plist
	/usr/libexec/PlistBuddy -x -c "Delete :com.apple.developer.team-identifier" ./RustWrapper.app/entitlements.plist
device-add-entitlements:
	/usr/libexec/PlistBuddy -x -c "Add :application-identifier string $(TEAM_ID).com.simlay.net.Dinghy" ./RustWrapper.app/entitlements.plist
	/usr/libexec/PlistBuddy -x -c "Add :com.apple.developer.team-identifier string $(TEAM_ID)" ./RustWrapper.app/entitlements.plist

device-sign: device-bundle
	make device-clear-entitlements
	make device-add-entitlements
	codesign -vvv -f -s "sebastian.imlay@gmail.com" --entitlements ./RustWrapper.app/entitlements.plist ./RustWrapper.app/
	codesign -vvv -d  --entitlements - --xml ./RustWrapper.app/

device-install: device-sign
	make xcrun-install

device-run: device-install
	make xcrun-run

xcrun-install:
	xcrun devicectl device install app --device $(DEVICE_ID) ./RustWrapper.app/

xcrun-run:
	xcrun devicectl device  process launch --device $(DEVICE_ID) com.simlay.net.Dinghy
