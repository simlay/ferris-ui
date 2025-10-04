use crate::{GUIEvent, View};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{DeclaredClass, MainThreadOnly, define_class, msg_send};
use objc2_foundation::{MainThreadMarker, NSObject, NSObjectProtocol, NSString};
use objc2_ui_kit::{
    UIColor, UIResponder, UIScrollViewDelegate, UITextView, UITextViewDelegate, UIView,
};
use std::cell::RefCell;
use winit::event_loop::EventLoopProxy;

pub struct TextViewState {
    delegate: RefCell<Retained<TextFieldDelegate>>,
    proxy: EventLoopProxy<GUIEvent>,
    event_fn: RefCell<Option<Box<dyn Fn(&TextView)>>>,
    place_holder_text: RefCell<Option<String>>,
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
        #[unsafe(method(textViewDidChangeSelection:))]
        fn did_change_selection(&self, _sender: &TextView) {}
        #[unsafe(method(textViewDidBeginEditing:))]
        fn did_begin_editing(&self, sender: &TextView) {
            sender.began_editing();
        }

        #[unsafe(method(textViewDidEndEditing:))]
        fn did_end_editing(&self, sender: &TextView) {
            sender.ended_editing();
        }

        #[unsafe(method(textViewDidChange:))]
        fn did_change(&self, sender: &TextView) {
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
            place_holder_text: RefCell::new(None),
        });
        let this: Retained<TextView> = unsafe { msg_send![super(this), init] };
        {
            let delegate = this.ivars().delegate.borrow();
            unsafe {
                this.setDelegate(Some(ProtocolObject::from_ref(&*delegate.clone())));
            }
        }

        this
    }

    pub fn get_text(&self) -> String {
        self.text().to_string()
    }

    fn began_editing(&self) {
        self.setText(None);
        self.setTextColor(Some(&UIColor::blackColor()));
    }

    fn ended_editing(&self) {
        let place_holder_text = self
            .ivars()
            .place_holder_text
            .borrow()
            .clone()
            .unwrap_or_default();
        if !place_holder_text.is_empty() {
            self.setText(Some(&NSString::from_str(place_holder_text.as_str())));
            self.setTextColor(Some(&UIColor::grayColor()));
        }
    }

    fn text_changed(&self) {
        let text = self.get_text();

        let _ = self.ivars().proxy.send_event(GUIEvent::Text(text.clone()));

        if let Some(event_fn) = &*self.ivars().event_fn.borrow() {
            event_fn(self);
        }
    }

    pub fn with_place_holder_text(self: Retained<Self>, place_holder: String) -> Retained<Self> {
        *self.ivars().place_holder_text.borrow_mut() = Some(place_holder);
        self
    }
}

impl View for TextView {
    fn raw_view(&self) -> Box<&UIView> {
        self.ended_editing();
        Box::new(self.as_ref())
    }
    #[cfg(feature = "nightly")]
    fn with_event_fn(self: Retained<Self>, event_fn: Box<dyn Fn(&Self)>) -> Retained<Self> {
        *self.ivars().event_fn.borrow_mut() = Some(event_fn);
        self
    }
}
