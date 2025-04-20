build:
	swift build -vvv  \
		-Xswiftc -sdk -Xswiftc $(shell xcrun --sdk iphonesimulator --show-sdk-path) \
		-Xcc -isysroot -Xcc $(shell xcrun --sdk iphonesimulator --show-sdk-path) \
		-Xswiftc -target -Xswiftc arm64-apple-ios18.0-simulator

EMULATOR='iPhone 16 Pro'

bundle: build
	cp .build/debug/swift-example ../RustWrapper.app/simple

install: bundle
	xcrun simctl install $(EMULATOR) ../RustWrapper.app/

run: install
	xcrun simctl launch --console --terminate-running-process $(EMULATOR) com.simlay.net.Dinghy

screenshot: install
	xcrun simctl launch \
		--stdout=$(PWD)/stdout.txt \
		--stderr=$(PWD)/stderr.txt \
		--terminate-running-process $(EMULATOR) RustWrapper
	sleep 10
	xcrun simctl io $(EMULATOR) screenshot swift-screenshot.png
	sips -Z 1278 swift-screenshot.png

watch:
	watchexec -r -w ./Sources/main.swift -w Package.swift 'make swift-example-run'

build-device:
	swift build -vvv  \
		-Xswiftc -sdk -Xswiftc $(shell xcrun --sdk iphoneos --show-sdk-path) \
		-Xcc -isysroot -Xcc $(shell xcrun --sdk iphoneos --show-sdk-path) \
		-Xswiftc -target -Xswiftc arm64-apple-ios18.0

bundle-device: build-device
	cp .build/debug/swift-example ../RustWrapper.app/simple

sign-device: bundle-device
	make -C ../ device-clear-entitlements
	make -C ../ device-add-entitlements
	codesign -vvv -f -s "sebastian.imlay@gmail.com" --entitlements ../RustWrapper.app/entitlements.plist ../RustWrapper.app/
	codesign -vvv -d  --entitlements - --xml ../RustWrapper.app/

device-install: sign-device
	make -C ../ xcrun-install

device-run: device-install
	make -C ../ xcrun-run
