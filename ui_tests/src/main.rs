
use ferris_ui::objc2::{define_class, ClassType, MainThreadOnly};
use objc2_xc_test::XCTestCase;
use objc2_xc_ui_automation::{
    XCUIApplication, XCUIElementTypeQueryProvider, XCUIScreenshotProviding,
};
use objc2_foundation::NSString;
use objc2_foundation::{
    NSSearchPathDomainMask,
    NSSearchPathDirectory,
    NSSearchPathForDirectoriesInDomains,
};

define_class!(
    #[unsafe(super = XCTestCase)]
    #[thread_kind = MainThreadOnly]
    struct TestCase;

    impl TestCase {
        #[unsafe(method(setUp))]
        fn set_up(&self) {
            println!("THIS IS THE SETUP");
            // Test setup code in here.
        }

        #[unsafe(method(tearDown))]
        fn tear_down(&self) {
            // Test teardown code in here.
            println!("THIS IS THE TEARDOWN");
        }

        #[unsafe(method(testScreenshot))]
        fn test_screenshot(&self) {
            println!("GETTING APPLICATION");
            let app = unsafe { XCUIApplication::new(self.mtm()) };
            println!("LAUNCHING APPLICATION");
            unsafe { app.launch() };
            println!("TAKING A SCREENSHOT");
            // Save screenshot.
            let screenshot = unsafe { app.windows().element().screenshot() };
            println!("TOOK A SCREENSHOT");
            println!("SAVING SCREENSHOT");
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
                let res = unsafe {
                    screenshot
                        .PNGRepresentation()
                        .writeToFile_atomically(&NSString::from_str(path.to_str().unwrap()), false)
                };
                assert!(res, "failed writing screenshot");
                //let mut output = File::create(path).unwrap();
            }

            // Run your test code in here.

            // TODO: For some reason, we have to terminate the application
            // manually when running outside Xcode?
            unsafe { app.terminate() };
        }
    }
);
fn main() { }

/// Load and initialize the class such that XCTest can see it.
#[ctor::ctor]
unsafe fn setup() {
    let _ = TestCase::class();
}
