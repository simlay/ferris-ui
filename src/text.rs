use crate::View;
use objc2::rc::Retained;
use objc2::{MainThreadMarker, MainThreadOnly, define_class, msg_send};
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
    pub fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = mtm.alloc().set_ivars(());
        let this: Retained<Self> = unsafe { msg_send![super(this), init] };
        this
    }
    pub fn set_text<T: Into<String>>(&self, new_text: T) {
        // UNANSWERED: Is this safe?
        unsafe {
            self.setText(Some(&NSString::from_str(&new_text.into())));
        }
    }
    pub fn with_text<T: Into<String>>(self: Retained<Self>, new_text: T) -> Retained<Self> {
        self.set_text(new_text);
        self
    }
    pub fn clear_text(&self) {
        unsafe {
            self.setText(None);
        }
    }
}
impl View for Text {
    fn ui_view(&self) -> Box<&UIView> {
        Box::new(self.as_ref())
    }
}
