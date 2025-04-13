use crate::{GUIEvent, View};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{DeclaredClass, MainThreadOnly, define_class, msg_send};
use objc2_foundation::{MainThreadMarker, NSObject, NSObjectProtocol, NSAttributedString};
use objc2_ui_kit::{UIResponder, UIScrollViewDelegate, UITextView, UITextViewDelegate, UIView};
use std::cell::RefCell;
use winit::event_loop::EventLoopProxy;

pub struct TextViewState {
    delegate: RefCell<Retained<TextFieldDelegate>>,
    proxy: EventLoopProxy<GUIEvent>,
    event_fn: RefCell<Option<Box<dyn Fn(&TextView)>>>,
}

define_class!(
    #[unsafe(super(UITextView, UIView, UIResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "FerrisUITextView"]
    #[ivars = TextViewState]
    pub struct TextView;

    impl TextView { }
);

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "FerrisTextViewDelegate"]
    pub struct TextFieldDelegate;

    unsafe impl NSObjectProtocol for TextFieldDelegate {}
    unsafe impl UIScrollViewDelegate for TextFieldDelegate {}
    unsafe impl UITextViewDelegate for TextFieldDelegate {
        #[unsafe(method(textViewDidBeginEditing:))]
        fn text_field_did_begin_editing(&self, _sender: &TextView) {
            /*
            let text = sender.text();
            println!("DidBeginEditing: {text}");
            */
        }

        #[unsafe(method(textViewDidEndEditing:))]
        fn text_field_did_end_editing(&self, _sender: &TextView) {
            /*
            let text = sender.text();
            println!("DidEndEditing: {text}");
            */
        }

        #[unsafe(method(textViewDidChange:))]
        fn text_field_did_change(&self, sender: &TextView) {
            /*
            let text = sender.text();
            println!("textViewDidChange: {text}");
            */
            sender.text_changed();
        }
    }
);

impl TextView {
    pub fn new(mtm: MainThreadMarker, proxy: EventLoopProxy<GUIEvent>) -> Retained<Self> {
        let delegate: Retained<TextFieldDelegate> =
            unsafe { objc2::msg_send![TextFieldDelegate::alloc(mtm), init] };
        let this = Self::alloc(mtm).set_ivars(TextViewState {
            delegate: RefCell::new(delegate),
            proxy,
            event_fn: RefCell::new(None),
        });
        let this: Retained<TextView> = unsafe { msg_send![super(this), init] };
        {
            let delegate = this.ivars().delegate.borrow();
            unsafe { this.setDelegate(Some(ProtocolObject::from_ref(&*delegate.clone()))) };
        }
        let alt_text = NSAttributedString::new();
        unsafe {
            this.setAttributedText(Some(&alt_text));
        }

        this
    }

    pub fn get_text(&self) -> String {
        // UNANSWERED: Is this safe?
        unsafe { self.text() }.to_string()
    }

    fn text_changed(&self) {
        let text = self.get_text();
        let _ = self.ivars().proxy.send_event(GUIEvent::Text(text.clone()));
        if let Some(event_fn) = &*self.ivars().event_fn.borrow() {
            event_fn(self);
        }
    }
}

impl View for TextView {
    fn ui_view(&self) -> Box<&UIView> {
        Box::new(self.as_ref())
    }
    #[cfg(feature = "nightly")]
    fn with_event_fn(self: Retained<Self>, event_fn: Box<dyn Fn(&Self)>) -> Retained<Self> {
        *self.ivars().event_fn.borrow_mut() = Some(event_fn);
        self
    }
}
