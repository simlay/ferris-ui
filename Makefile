EMULATOR='iPhone 16e'
OTHER_EMULATOR='iPhone 16e'
DEVICE_ID=aoeu
TEAM_ID=aoeu

WRAPPER_ID=com.simlay.net.Dinghy
XCWRAPPER_ID=com.simlay.net.DinghyUITests.xctrunner

GET_CONTAINER=xcrun simctl get_app_container $(EMULATOR)
DINGHY_CONTAINER_CMD=$(GET_CONTAINER) $(WRAPPER_ID)
DINGHY_CONTAINER=$(shell $(DINGHY_CONTAINER_CMD) data)

XCTEST_CONTAINER_CMD=$(GET_CONTAINER) $(XCWRAPPER_ID)
XCTEST_CONTAINER=$(shell $(XCTEST_CONTAINER_CMD) data)
LAUNCH=xcrun simctl launch --console --terminate-running-process $(EMULATOR)

#LLVM_COV=xcrun llvm-cov
#LLVM_PROFDATA=xcrun llvm-profdata

LLVM_COV=$(shell rustc --print sysroot)/lib/rustlib/aarch64-apple-darwin/bin/llvm-cov
LLVM_PROFDATA=$(shell rustc --print sysroot)/lib/rustlib/aarch64-apple-darwin/bin/llvm-profdata

COV_IGNORE_LIST=--ignore-filename-regex='/.cargo/registry' --ignore-filename-regex='/.rustup' --ignore-filename-regex='/objc2'

COV_REPORT=$(LLVM_COV) report -Xdemangler=rustfilt --use-color   $(COV_IGNORE_LIST) -instr-profile
COV_EXPORT=$(LLVM_COV) export -Xdemangler=rustfilt --format lcov $(COV_IGNORE_LIST) -instr-profile


build:
	cargo build --target aarch64-apple-ios-sim --all --all-targets

bundle: build
	cp ./target/aarch64-apple-ios-sim/debug/examples/simple ./RustWrapper.app/

install: bundle
	[ "$(shell md5sum --quiet $(shell $(DINGHY_CONTAINER_CMD))/simple)" = "$(shell md5sum --quiet RustWrapper.app/simple)" ] || xcrun simctl install $(EMULATOR) ./RustWrapper.app/

debug: install
	./openDebug.scpt $(shell xcrun simctl launch --stdout=$(PWD)/stdout.txt --stderr=$(PWD)/stderr.txt --terminate-running-process $(EMULATOR) $(WRAPPER_ID) | awk '{print $$2}')

debug-wait: install
	xcrun simctl launch --wait-for-debugger --console --terminate-running-process $(EMULATOR) $(WRAPPER_ID)

run: install
	SIMCTL_CHILD_LLVM_PROFILE_FILE="$(DINGHY_CONTAINER)/Documents/dinghy.profraw" $(LAUNCH) $(WRAPPER_ID)

CURR_EMULATOR:=$(shell cat ./target/emulator)
run-no-wait: bundle
	xcrun simctl install "$(CURR_EMULATOR)" ./RustWrapper.app/
	SIMCTL_CHILD_RUST_BACKTRACE=full SIMCTL_CHILD_RUST_LOG=trace xcrun simctl launch "$(CURR_EMULATOR)" $(WRAPPER_ID)
	if [ -z "$(CURR_EMULATOR)" ]; then echo $(OTHER_EMULATOR) > ./target/emulator; fi
	if [ "$(CURR_EMULATOR)" = $(EMULATOR) ]; then echo $(OTHER_EMULATOR) > ./target/emulator; fi
	if [ "$(CURR_EMULATOR)" = $(OTHER_EMULATOR) ]; then echo $(EMULATOR) > ./target/emulator; fi


watch-swap:
	cargo watch -s 'make run-no-wait' -w ./src -w ./Cargo.toml -w ./examples/

watch:
	cargo watch -s 'make run' -w ./src -w ./Cargo.toml -w ./examples/

screenshot: install
	xcrun simctl launch --stdout=$(PWD)/stdout.txt --stderr=$(PWD)/stderr.txt --terminate-running-process $(EMULATOR) $(WRAPPER_ID)
	sleep 2
	xcrun simctl io $(EMULATOR) screenshot screenshot.png
	#sips -Z 1278 screenshot.png

record-and-stop: install
	xcrun simctl launch --stdout=$(PWD)/stdout.txt --stderr=$(PWD)/stderr.txt --terminate-running-process $(EMULATOR) $(WRAPPER_ID) --record
	xcrun simctl io $(EMULATOR) recordVideo -f record.mp4 &
	sleep 2
	ps | grep 'simctl io $(EMULATOR)  recordVideo' | grep -v grep | awk '{print $$1}' | xargs kill -s SIGINT

record:
	xcrun simctl io $(EMULATOR) recordVideo -f record.mp4

compress-recording:
	ffmpeg -i record.mp4 -c:v libx264 -crf 30 record-compressed.mp4

gh-summary:
	@touch stdout.txt stderr.txt
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

build-macabi:
	cargo build --target aarch64-apple-ios-macabi -Zbuild-std --all

bundle-macabi: build-macabi
	cp ./target/aarch64-apple-ios-macabi/debug/examples/simple ./RustWrapper.app/

run-macabi: bundle-macabi
	open ./RustWrapper.app/

watch-macabi:
	cargo watch -s 'make run-macabi' -w ./src -w ./Cargo.toml -w ./examples/

device-build:
	cargo build --target aarch64-apple-ios --all --all-targets

device-bundle: device-build device-add-entitlements
	cp ./target/aarch64-apple-ios/debug/examples/simple ./RustWrapper.app/
	cp ./target/aarch64-apple-ios/debug/ui_tests ./RustUITests-Runner.app/PlugIns/DinghyUITests.xctest/

device-install: sign-bundle
	xcrun devicectl device install app --device $(DEVICE_ID) ./RustWrapper.app/

device-run: sign-bundle device-install
	xcrun devicectl device  process launch --device $(DEVICE_ID) $(WRAPPER_ID)

device-clear-entitlements:
	/usr/libexec/PlistBuddy -x -c "Delete :application-identifier"              ./RustWrapper.app/entitlements.plist || true
	/usr/libexec/PlistBuddy -x -c "Delete :com.apple.developer.team-identifier" ./RustWrapper.app/entitlements.plist || true
	/usr/libexec/PlistBuddy -x -c "Delete :application-identifier"              ./RustUITests-Runner.app/entitlements.plist || true
	/usr/libexec/PlistBuddy -x -c "Delete :com.apple.developer.team-identifier" ./RustUITests-Runner.app/entitlements.plist || true

device-add-entitlements: device-clear-entitlements
	/usr/libexec/PlistBuddy -x -c "Add :application-identifier string $(TEAM_ID).$(WRAPPER_ID)" ./RustWrapper.app/entitlements.plist
	/usr/libexec/PlistBuddy -x -c "Add :com.apple.developer.team-identifier string $(TEAM_ID)"  ./RustWrapper.app/entitlements.plist
	/usr/libexec/PlistBuddy -x -c "Add :application-identifier string $(TEAM_ID).$(XCWRAPPER_ID)" ./RustUITests-Runner.app/entitlements.plist
	/usr/libexec/PlistBuddy -x -c "Add :com.apple.developer.team-identifier string $(TEAM_ID)"  ./RustUITests-Runner.app/entitlements.plist

sign-bundle: device-bundle device-add-entitlements
	codesign -vvv -f -s "sebastian.imlay@gmail.com" --entitlements ./RustWrapper.app/entitlements.plist ./RustWrapper.app/
	codesign -vvv -d  --entitlements - --xml ./RustWrapper.app/
	codesign -vvv -f -s "sebastian.imlay@gmail.com" --entitlements ./RustUITests-Runner.app/entitlements.plist ./RustUITests-Runner.app/
	codesign -vvv -d  --entitlements - --xml ./RustUITests-Runner.app/

ui-tests-build:
	cargo build -p ui_tests

ui-tests-bundle-tools: ui-tests-build
	cp $(shell xcode-select --print-path)/Platforms/IPhoneSimulator.platform/Developer/Library/Xcode/Agents/XCTRunner.app/XCTRunner                ./RustUITests-Runner.app/
	cp -r $(shell xcode-select --print-path)/Platforms/IPhoneSimulator.platform/Developer/Library/Frameworks                                       ./RustUITests-Runner.app/
	cp -r $(shell xcode-select --print-path)/Platforms/IPhoneSimulator.platform/Developer/Library/PrivateFrameworks/XCTestCore.framework           ./RustUITests-Runner.app/Frameworks/
	cp -r $(shell xcode-select --print-path)/Platforms/IPhoneSimulator.platform/Developer/Library/PrivateFrameworks/XCTestSupport.framework        ./RustUITests-Runner.app/Frameworks/
	cp -r $(shell xcode-select --print-path)/Platforms/IPhoneSimulator.platform/Developer/Library/PrivateFrameworks/XCUnit.framework               ./RustUITests-Runner.app/Frameworks/
	cp -r $(shell xcode-select --print-path)/Platforms/IPhoneSimulator.platform/Developer/Library/PrivateFrameworks/XCTAutomationSupport.framework ./RustUITests-Runner.app/Frameworks/

ui-tests-bundle: ui-tests-bundle-tools
	cp ./target/aarch64-apple-ios-sim/debug/ui_tests  ./RustUITests-Runner.app/Plugins/DinghyUITests.xctest/

ui-tests-install: ui-tests-bundle
	#@xcrun simctl uninstall $(EMULATOR) $(XCWRAPPER_ID)
	@xcrun simctl install $(EMULATOR) RustUITests-Runner.app/

ui-tests-xctest-configuration: ui-tests-install
	cat ui_tests/ui_tests.xctestconfiguration.base | \
		sed "s:UI_TEST_WRAPPER_CONTAINER:$(shell $(XCTEST_CONTAINER_CMD)):g" | \
		sed "s:WRAPPER_APP_CONTAINER:$(shell $(DINGHY_CONTAINER_CMD)):g" | \
		sed "s:WRAPPER_APP_ID:$(WRAPPER_ID):g" \
		> ui_tests/ui_tests.xctestconfiguration

ui-tests-run: install ui-tests-install ui-tests-xctest-configuration
	SIMCTL_CHILD_DINGHY_LLVM_PROFILE_FILE="$(DINGHY_CONTAINER)/Documents/dinghy.profraw" SIMCTL_CHILD_LLVM_PROFILE_FILE="$(XCTEST_CONTAINER)/Documents/xctrunner.profraw" $(LAUNCH) $(XCWRAPPER_ID) 2>&1 | tee $(PWD)/stdout.txt
	make ui-tests-cp-screenshot

ui-tests-cp-screenshot:
	cp "$(XCTEST_CONTAINER)/Documents/screenshot.png" ui_tests.png
	sips -Z 1278 ui_tests.png

ui-tests-cov: ui-tests-run
	mkdir -p target/cov
	cp "$(XCTEST_CONTAINER)/Documents/xctrunner.profraw" ./target/cov/xctrunner.profraw
	cp "$(DINGHY_CONTAINER)/Documents/dinghy.profraw" ./target/cov/dinghy.profraw
	du -hs ./target/cov/*.profraw
	$(LLVM_PROFDATA)  merge -sparse ./target/cov/xctrunner.profraw -o ./target/cov/xctrunner.profdata
	$(LLVM_PROFDATA)  merge -sparse    ./target/cov/dinghy.profraw -o ./target/cov/dinghy.profdata
	$(COV_REPORT) ./target/cov/xctrunner.profdata ./target/aarch64-apple-ios-sim/debug/ui_tests
	$(COV_REPORT) ./target/cov/dinghy.profdata    ./target/aarch64-apple-ios-sim/debug/examples/simple
	$(COV_EXPORT) ./target/cov/xctrunner.profdata ./target/aarch64-apple-ios-sim/debug/ui_tests        > ./target/cov/xctrunner-lcov.info
	$(COV_EXPORT) ./target/cov/dinghy.profdata    ./target/aarch64-apple-ios-sim/debug/examples/simple > ./target/cov/dinghy-lcov.info
	genhtml ./target/cov/dinghy-lcov.info ./target/cov/xctrunner-lcov.info -o ./target/cov/
	# This could be for merging test coverage from `cargo test`
	#cp "$(shell xcrun simctl get_app_container $(EMULATOR) Dinghy data)/Documents/tests.profraw" ./target/cov/tests.profraw
	#xcrun llvm-profdata merge -sparse    ./target/cov/tests.profraw  -o ./target/cov/tests.profdata
	#$(COV_REPORT) ./target/cov/tests.profdata    ./target/dinghy/ferris_ui-51928494b02468a5/ferris-ui/Dinghy.app/Dinghy

install-swift:
	make -C ./swift-example/ install

ui-tests-run-swift: install-swift ui-tests-install ui-tests-xctest-configuration
	SIMCTL_CHILD_XCTestConfigurationFilePath=$(PWD)/ui_tests/ui_tests.xctestconfiguration \
		xcrun simctl launch --console $(EMULATOR) $(XCWRAPPER_ID) 2>&1 | tee $(PWD)/stdout.txt
	make ui-tests-cp-screenshot

.EXPORT_ALL_VARIABLES:
SIMCTL_CHILD_RUST_BACKTRACE=full
SIMCTL_CHILD_RUST_LOG=trace
SIMCTL_CHILD_XCTestConfigurationFilePath=$(PWD)/ui_tests/ui_tests.xctestconfiguration
SIMCTL_CHILD_LLVM_PROFILE_VERBOSE_ERRORS=1
