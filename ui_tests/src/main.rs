use ferris_ui::objc2::{ClassType, MainThreadOnly, define_class};
use objc2_foundation::NSString;
use objc2_foundation::{
    NSSearchPathDirectory, NSSearchPathDomainMask, NSSearchPathForDirectoriesInDomains,
};
use objc2_xc_test::XCTestCase;
use objc2_xc_ui_automation::{
    XCUIApplication, XCUIApplicationState, XCUIDevice, XCUIElement, XCUIElementTypeQueryProvider,
    XCUIScreenshot, XCUIScreenshotProviding,
    XCUIElementSnapshot,
    XCUICoordinate,
};

define_class!(
    #[unsafe(super = XCTestCase)]
    #[thread_kind = MainThreadOnly]
    struct TestCase;

    impl TestCase {
        #[unsafe(method(setUp))]
        fn set_up(&self) {
            // Test setup code in here.
        }

        #[unsafe(method(tearDown))]
        fn tear_down(&self) {
            // Test teardown code in here.
            //let app = XCUIApplication::new(self.mtm());
            //app.terminate();
        }

        #[unsafe(method(testScreenshot))]
        fn test_screenshot(&self) {
            let app = XCUIApplication::new(self.mtm());
            let device = XCUIDevice::sharedDevice(self.mtm());
            take_screenshot(&app, &device);
            // Run your test code in here.

        }
    }
);
fn main() {}

fn take_screenshot(app: &XCUIApplication, device: &XCUIDevice) {

    //println!("LAUNCHING APPLICATION");
    use objc2_foundation::{NSDictionary, ns_string};

    // SIMCTL_CHILD_DINGHY_LLVM_PROFILE_FILE
    if let Ok(val) = std::env::var("DINGHY_LLVM_PROFILE_FILE") {
        let foo: ferris_ui::objc2::rc::Retained<NSDictionary<NSString, NSString>> =
            NSDictionary::from_slices(
                &[
                    ns_string!("LLVM_PROFILE_FILE"),
                    ns_string!("LLVM_PROFILE_VERBOSE_ERRORS"),
                ],
                &[&NSString::from_str(val.as_str()), ns_string!("1")],
            );

        app.setLaunchEnvironment(&foo);
    }
    //println!("APPLICATION LAUNCH ENV: {:?}", app.launchEnvironment());
    app.launch();
    device.setOrientation(ferris_ui::objc2_ui_kit::UIDeviceOrientation::Portrait);

    save_screenshot(&app.screenshot(), "screenshot".into());
    let text_view = app.textViews().element();
    text_view.tap();
    text_view.typeText(&NSString::from_str("THIS IS SOME TEXT"));

    let siri = device.siriService();
    siri.activateWithVoiceRecognitionText(&NSString::from_str("What is the capital of germany?"));

    std::thread::sleep_ms(1000);

    save_screenshot(&app.screenshot(), "screenshot".into());
    device.pressButton(objc2_xc_ui_automation::XCUIDeviceButton::Home);
    press_home(&device);
}

fn press_home(device: &XCUIDevice) {
    unsafe {
        let _: () = ferris_ui::objc2::msg_send![device, pressButton: 1_isize];
    };
}

fn save_screenshot(screenshot: &XCUIScreenshot, basename: String) {
    let path = NSSearchPathForDirectoriesInDomains(
        NSSearchPathDirectory::DocumentDirectory,
        NSSearchPathDomainMask::UserDomainMask,
        true,
    );
    if let Some(path) = path.firstObject() {
        let path = path.to_string();
        let path = std::path::Path::new(&path).join(format!("{basename}.png"));
        println!("PATH IS {path:?}");
        let res = screenshot
            .PNGRepresentation()
            .writeToFile_atomically(&NSString::from_str(path.to_str().unwrap()), false);
        assert!(res, "failed writing screenshot");
        //let mut output = File::create(path).unwrap();
    }
}

/// Load and initialize the class such that XCTest can see it.
#[ctor::ctor]
unsafe fn setup() {
    let _ = TestCase::class();
}
