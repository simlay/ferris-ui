use crate::{GUIEvent, View};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{DeclaredClass, MainThreadOnly, define_class, msg_send};
use objc2_foundation::{MainThreadMarker, NSObject, NSObjectProtocol, NSString};
use objc2_ui_kit::{UIResponder, UIScrollViewDelegate, UITextField, UITextFieldDelegate, UIView};
use std::cell::RefCell;
use winit::event_loop::EventLoopProxy;

pub struct TextFieldState {
    delegate: RefCell<Retained<TextFieldDelegate>>,
    proxy: EventLoopProxy<GUIEvent>,
    event_fn: RefCell<Option<Box<dyn Fn(&TextField)>>>,
}

define_class!(
    #[unsafe(super(UITextField, UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "FerrisUITextField"]
    #[ivars = TextFieldState]
    pub struct TextField;

    impl TextField { }
);

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "FerrisTextFieldDelegate"]
    pub struct TextFieldDelegate;

    unsafe impl NSObjectProtocol for TextFieldDelegate {}
    unsafe impl UIScrollViewDelegate for TextFieldDelegate {}
    unsafe impl UITextFieldDelegate for TextFieldDelegate {
        #[unsafe(method(textFieldDidBeginEditing:))]
        fn text_field_did_begin_editing(&self, sender: &TextField) {
            let text = sender.get_text();
            println!("DidBeginEditing: {text:?}");
            /*
             */
        }

        #[unsafe(method(textFieldDidEndEditing:))]
        fn text_field_did_end_editing(&self, sender: &TextField) {
            sender.text_changed();
            let text = sender.get_text();
            println!("DidEndEditing: {text:?}");
            /*
            */
        }

        /*
        #[unsafe(method(textFieldDidChange:))]
        fn text_field_did_change(&self, sender: &TextField) {
            let text = sender.get_text();
            println!("textViewDidChange: {text:?}");
            sender.text_changed();
        }
        */
    }
);

impl TextField {
    pub fn new(mtm: MainThreadMarker, proxy: EventLoopProxy<GUIEvent>) -> Retained<Self> {
        let delegate: Retained<TextFieldDelegate> =
            unsafe { objc2::msg_send![TextFieldDelegate::alloc(mtm), init] };
        let this = Self::alloc(mtm).set_ivars(TextFieldState {
            delegate: RefCell::new(delegate),
            proxy,
            event_fn: RefCell::new(None),
        });
        let this: Retained<TextField> = unsafe { msg_send![super(this), init] };
        {
            let delegate = this.ivars().delegate.borrow();
            unsafe { this.setDelegate(Some(ProtocolObject::from_ref(&*delegate.clone()))) };
        }
        unsafe { this.setPlaceholder(Some(&NSString::from_str("THIS IS SOME INPUT"))) };

        this
    }

    pub fn get_text(&self) -> Option<String> {
        unsafe { self.text() }.map(|t| t.to_string())
    }

    fn text_changed(&self) {
        let text = self.get_text();
        let _ = self
            .ivars()
            .proxy
            .send_event(GUIEvent::Text(text.unwrap_or_default().clone()));
        if let Some(event_fn) = &*self.ivars().event_fn.borrow() {
            event_fn(self);
        }
    }
}

impl View for TextField {
    fn ui_view(&self) -> Box<&UIView> {
        Box::new(self.as_ref())
    }
    #[cfg(feature = "nightly")]
    fn with_event_fn(self: Retained<Self>, event_fn: Box<dyn Fn(&Self)>) -> Retained<Self> {
        *self.ivars().event_fn.borrow_mut() = Some(event_fn);
        self
    }
}
