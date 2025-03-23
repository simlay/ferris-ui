use objc2::rc::Retained;
use objc2::{msg_send, define_class, MainThreadOnly, MainThreadMarker};
use objc2_foundation::{NSObject, NSString};
use objc2_ui_kit::{UILabel, UIView};

define_class!(
    #[unsafe(super(UILabel, UIView, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "FerrisUILabel"]
    pub struct Text;

    impl Text { }
);

impl Text {
    pub fn new() -> Retained<Self> {
        let mtm = MainThreadMarker::new().unwrap();
        let this = mtm.alloc().set_ivars(());
        let this: Retained<Self> = unsafe { msg_send![super(this), init] };
        this
    }
    pub fn set_text<T: Into<String>>(&self, new_text: T) {
        unsafe {
            self.setText(Some(&NSString::from_str(&new_text.into())));
        }
    }
    pub fn clear_text(&self) {
        unsafe {
            self.setText(None);
        }
    }
}
