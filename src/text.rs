use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{declare_class, msg_send_id, mutability, ClassType, DeclaredClass};
use objc2_foundation::{
    CGPoint, CGRect, CGSize, MainThreadMarker, NSObject, NSObjectProtocol, NSString,
};
use objc2_ui_kit::{
    UIColor, UIControlEvents, UIEdgeInsets, UILabel, UIResponder, UISwitch, UIToolbar, UIView,
};
use std::cell::RefCell;

use crate::GUIEvent;
use winit::event_loop::EventLoopProxy;

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
    pub fn set_text(&self, new_text: String) {
        unsafe {
            self.setText(Some(&NSString::from_str(&new_text)));
        }
    }
    pub fn clear_text(&self) {
        unsafe {
            self.setText(None);
        }
    }
}
