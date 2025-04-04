swift-example-build:
	swift build -vvv  \
		-Xswiftc "-sdk" -Xswiftc "$(shell xcrun --sdk iphonesimulator --show-sdk-path)" \
		-Xcc "-isysroot" -Xcc "$(shell xcrun --sdk iphonesimulator --show-sdk-path)" \
		-Xswiftc "-target" -Xswiftc "x86_64-apple-ios18.0-simulator" \

swift-example-bundle: swift-example-build
	cp .build/debug/swift-example ../RustWrapper.app/simple

swift-example-install: swift-example-bundle
	xcrun simctl install booted ../RustWrapper.app/

swift-example-run: swift-example-install
	xcrun simctl launch --console --terminate-running-process booted RustWrapper

swift-example-screenshot: swift-example-install
	xcrun simctl launch \
		--stdout=$(PWD)/stdout.txt \
		--stderr=$(PWD)/stderr.txt \
		--terminate-running-process booted RustWrapper
	sleep 5
	xcrun simctl io booted screenshot swift-screenshot.png
	sips -Z 1278 swift-screenshot.png
watch:
	watchexec -r -w ./Sources/main.swift -w Package.swift 'make swift-example-run'
