use std::cell::RefCell;
use objc2_foundation::{
    CGPoint, CGRect, CGSize, MainThreadMarker, NSObject, NSObjectProtocol,
};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{declare_class, msg_send_id, mutability, ClassType, DeclaredClass };
use objc2_ui_kit::{
    UITextView, UIView, UIResponder, UITextViewDelegate, UIScrollViewDelegate,
};
use winit::event_loop::EventLoopProxy;
use crate::GUIEvent;

pub struct TextFieldState {
    delegate: RefCell<Retained<TextFieldDelegate>>,
    proxy: EventLoopProxy<GUIEvent>,
}

declare_class!(
    pub struct TextField;
    unsafe impl ClassType for TextField {
        #[inherits(UIView, UIResponder, NSObject)]
        type Super = UITextView;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "FerrisUITextView";
    }

    impl DeclaredClass for TextField {
        type Ivars = TextFieldState;
    }

    unsafe impl TextField { }
);

declare_class!(
    pub struct TextFieldDelegate;

    unsafe impl ClassType for TextFieldDelegate {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "FerrisTextViewDelegate";
    }

    impl DeclaredClass for TextFieldDelegate {
        type Ivars = ();
    }

    unsafe impl NSObjectProtocol for TextFieldDelegate {}
    unsafe impl UIScrollViewDelegate for TextFieldDelegate {}
    unsafe impl UITextViewDelegate for TextFieldDelegate {
        #[method(textViewDidBeginEditing:)]
        unsafe fn text_field_did_begin_editing(&self, sender: &TextField) {
            let text = sender.text();
            println!("DidBeginEditing: {text}");
        }

        #[method(textViewDidEndEditing:)]
        unsafe fn text_field_did_end_editing(&self, sender: &TextField) {
            let text = sender.text();
            println!("DidEndEditing: {text}");
        }

        #[method(textViewDidChange:)]
        unsafe fn text_field_did_change(&self, sender: &TextField) {
            let text = sender.text();
            println!("textViewDidChange: {text}");
            sender.text_changed();
        }
    }
);

impl TextField {
    pub fn new(mtm: MainThreadMarker, proxy: EventLoopProxy<GUIEvent>) -> Retained<Self> {

        // TODO: This should be hidden someplace.
        let delegate: Retained<TextFieldDelegate> = unsafe { objc2::msg_send_id![mtm.alloc(), init]};
        let this = mtm.alloc().set_ivars(TextFieldState {
            delegate: RefCell::new(delegate),
            proxy,
        });
        let this: Retained<TextField> = unsafe { msg_send_id![super(this), init] };

        {
            let delegate = this.ivars().delegate.borrow();
            unsafe {this.setDelegate(Some(ProtocolObject::from_ref(&*delegate.clone())))};
        }

        this
    }
    fn text_changed(&self) {
        let text = unsafe{self.text()}.to_string();
        let _ = self.ivars().proxy.send_event(GUIEvent::Text(text));
    }
}