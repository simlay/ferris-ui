use crate::{GUIEvent, View};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{declare_class, msg_send_id, mutability, ClassType, DeclaredClass};
use objc2_foundation::{CGPoint, CGRect, CGSize, MainThreadMarker, NSObject, NSObjectProtocol};
use objc2_ui_kit::{UIResponder, UIScrollViewDelegate, UITextView, UITextViewDelegate, UIView};
use std::cell::RefCell;
use winit::event_loop::EventLoopProxy;

pub struct TextFieldState {
    delegate: RefCell<Retained<TextFieldDelegate>>,
    proxy: EventLoopProxy<GUIEvent>,
    event_fn: Option<Box<dyn Fn(&TextField)>>,
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
    pub fn new(
        proxy: EventLoopProxy<GUIEvent>,
        event_fn: Option<Box<dyn Fn(&Self)>>,
    ) -> Retained<Self> {
        let mtm = MainThreadMarker::new().unwrap();

        // TODO: This should be hidden someplace.
        let delegate: Retained<TextFieldDelegate> =
            unsafe { objc2::msg_send_id![mtm.alloc(), init] };
        let this = mtm.alloc().set_ivars(TextFieldState {
            delegate: RefCell::new(delegate),
            proxy,
            event_fn,
        });
        let this: Retained<TextField> = unsafe { msg_send_id![super(this), init] };
        {
            let delegate = this.ivars().delegate.borrow();
            unsafe { this.setDelegate(Some(ProtocolObject::from_ref(&*delegate.clone()))) };
        }

        this
    }

    fn text_changed(&self) {
        let text = unsafe { self.text() }.to_string();
        let _ = self.ivars().proxy.send_event(GUIEvent::Text(text.clone()));
        if let Some(event_fn) = &self.ivars().event_fn {
            event_fn(self);
        }
    }
}

impl View for TextField {
    fn ui_view(&self) -> Box<&UIView> {
        Box::new(self.as_ref())
    }
    #[cfg(feature = "nightly")]
    fn set_event_fn(self: Retained<Self>, event_fn: Box<dyn Fn(&Self)>) -> Retained<Self> {
        let ivars = self.ivars();
        Self::new(ivars.proxy.clone(), Some(event_fn))
    }
}
