use ferris_ui::objc2::{ClassType, MainThreadOnly, define_class};
use objc2_foundation::NSString;
use objc2_foundation::{
    NSSearchPathDirectory, NSSearchPathDomainMask, NSSearchPathForDirectoriesInDomains,
};
use objc2_xc_test::XCTestCase;
use objc2_xc_ui_automation::{
    XCUIApplication, XCUIApplicationState, XCUIDevice, XCUIElementTypeQueryProvider,
    XCUIScreenshot, XCUIScreenshotProviding,
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
    let system = device.system();

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

    let window = app.windows();
    let root_node = window.element();
    save_screenshot(&root_node.screenshot());
    //println!("LAYOUT: {layout}");
    //println!("LAYOUT: {}", root_node.debugDescription());
    let all_text_fields = root_node.textViews();
    let text_view = all_text_fields.elementBoundByIndex(1);
    println!("TEXT VIEW: {}", text_view.debugDescription());

    text_view.tap();
    text_view.typeText(&NSString::from_str("THIS IS SOME TEXT"));
    println!("PUT TEXT IN");
    unsafe {
        let _: () = ferris_ui::objc2::msg_send![device, pressButton: 1_isize];
    };
    println!("PRESSED HOME BUTTON");
    //std::thread::sleep_ms(2_000);

    // TODO: For some reason, we have to terminate the application
    // manually when running outside Xcode?
    //app.terminate();
}

fn save_screenshot(screenshot: &XCUIScreenshot) {
    //println!("TOOK A SCREENSHOT");
    //println!("SAVING SCREENSHOT");
    let path = NSSearchPathForDirectoriesInDomains(
        NSSearchPathDirectory::DocumentDirectory,
        NSSearchPathDomainMask::UserDomainMask,
        true,
    );
    if let Some(path) = path.firstObject() {
        let path = path.to_string();
        let path = std::path::Path::new(&path).join("screenshot.png");
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct XCUIDeviceButton(pub objc2_foundation::NSInteger);
impl XCUIDeviceButton {
    #[doc(alias = "XCUIDeviceButtonHome")]
    pub const Home: Self = Self(1);
    #[doc(alias = "XCUIDeviceButtonVolumeUp")]
    pub const VolumeUp: Self = Self(2);
    #[doc(alias = "XCUIDeviceButtonVolumeDown")]
    pub const VolumeDown: Self = Self(3);
    #[doc(alias = "XCUIDeviceButtonAction")]
    pub const Action: Self = Self(4);
    #[doc(alias = "XCUIDeviceButtonCamera")]
    pub const Camera: Self = Self(5);
}
use ferris_ui::objc2::{Encode, Encoding, RefEncode};

unsafe impl Encode for XCUIDeviceButton {
    const ENCODING: Encoding = objc2_foundation::NSInteger::ENCODING;
}

unsafe impl RefEncode for XCUIDeviceButton {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
