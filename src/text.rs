use objc2::rc::Retained;
use objc2::{ClassType, DeclaredClass, declare_class, msg_send_id, mutability};
use objc2_foundation::{MainThreadMarker, NSObject, NSString};
use objc2_ui_kit::{UILabel, UIView};

declare_class!(
    pub struct Text;
    unsafe impl ClassType for Text {
        #[inherits(UIView, NSObject)]
        type Super = UILabel;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "FerrisUILabel";
    }

    impl DeclaredClass for Text {
        type Ivars = ();
    }

    unsafe impl Text { }
);

impl Text {
    pub fn new() -> Retained<Self> {
        let mtm = MainThreadMarker::new().unwrap();
        let this = mtm.alloc().set_ivars(());
        let this: Retained<Self> = unsafe { msg_send_id![super(this), init] };
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
