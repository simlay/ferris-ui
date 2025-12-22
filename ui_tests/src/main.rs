use ferris_ui::objc2::{ClassType, MainThreadOnly, define_class};
use objc2_foundation::NSString;
use objc2_foundation::{
    NSSearchPathDirectory, NSSearchPathDomainMask, NSSearchPathForDirectoriesInDomains,
};
use objc2_xc_test::XCTestCase;
use objc2_xc_ui_automation::{
    XCUIApplication, XCUIElementTypeQueryProvider, XCUIScreenshotProviding,
    XCUIScreenshot,
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
        }

        #[unsafe(method(testScreenshot))]
        fn test_screenshot(&self) {
            let app = XCUIApplication::new(self.mtm());
            take_screenshot(&app);
            // Run your test code in here.

            // TODO: For some reason, we have to terminate the application
            // manually when running outside Xcode?
            app.terminate();
        }
    }
);
fn main() {}

fn take_screenshot(app: &XCUIApplication) {
    //println!("LAUNCHING APPLICATION");
    use objc2_foundation::{
        NSDictionary,
        ns_string,
    };

    // SIMCTL_CHILD_DINGHY_LLVM_PROFILE_FILE
    if let Ok(val) = std::env::var("DINGHY_LLVM_PROFILE_FILE") {

        let foo : ferris_ui::objc2::rc::Retained<NSDictionary<NSString, NSString>> = NSDictionary::from_slices(
            &[ns_string!("LLVM_PROFILE_FILE")],
            &[&NSString::from_str(val.as_str())],
        );

        app.setLaunchEnvironment(
            &foo
        );
    }
    //println!("APPLICATION LAUNCH ENV: {:?}", app.launchEnvironment());
    app.launch();

    let window = app.windows();
    let root_node = window.element();
    let screenshot = root_node.screenshot();
    save_screenshot(&screenshot);
    //println!("LAYOUT: {layout}");
    //println!("LAYOUT: {}", root_node.debugDescription());
    let all_text_fields = root_node.textViews();
    let text_view = all_text_fields.elementBoundByIndex(0);
    println!("TEXT VIEW: {}", text_view.debugDescription());

    text_view.tap();
    text_view.typeText(&NSString::from_str("exit"));

    //println!("TAKING A SCREENSHOT");
    app.terminate();
}

fn save_screenshot(screenshot: &XCUIScreenshot) {
    //println!("TOOK A SCREENSHOT");
    //println!("SAVING SCREENSHOT");
    let path =
        NSSearchPathForDirectoriesInDomains(
            NSSearchPathDirectory::DocumentDirectory,
            NSSearchPathDomainMask::UserDomainMask,
            true,
        );
    if let Some(path) = path.firstObject() {
        let path = path.to_string();
        let path = std::path::Path::new(&path).join("screenshot.png");
        println!("PATH IS {path:?}");
        let res =
            screenshot
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
