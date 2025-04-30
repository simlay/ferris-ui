EMULATOR='iPhone 16 Pro'

build:
	swift build -vvv  \
		-Xswiftc -sdk -Xswiftc $(shell xcrun --sdk iphonesimulator --show-sdk-path) \
		-Xcc -isysroot -Xcc $(shell xcrun --sdk iphonesimulator --show-sdk-path) \
		-Xswiftc -target -Xswiftc arm64-apple-ios18.0-simulator

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
		--terminate-running-process $(EMULATOR) com.simlay.net.Dinghy
	sleep 10
	xcrun simctl io $(EMULATOR) screenshot swift-screenshot.png
	sips -Z 1278 swift-screenshot.png

watch:
	watchexec -r -w ./Sources/main.swift -w Package.swift 'make run'

build-device:
	swift build -vvv  \
		-Xswiftc -sdk -Xswiftc $(shell xcrun --sdk iphoneos --show-sdk-path) \
		-Xcc -isysroot -Xcc $(shell xcrun --sdk iphoneos --show-sdk-path) \
		-Xswiftc -target -Xswiftc arm64-apple-ios18.0

bundle-device: build-device
	cp .build/debug/swift-example ../RustWrapper.app/simple
	make -C ../ sign-bundle

device-install: bundle-device
	make -C ../ xcrun-install

device-run: device-install
	make -C ../ xcrun-run
