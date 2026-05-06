use crate::{GUIEvent, View};
use std::cell::RefCell;

//#[derive(Clone)]
pub struct TextEditor {
    env: crate::EnvMarker,
    inner: crate::NativeBox<native::UITextView>,
    text: String,
    pub event_fn: RefCell<Option<Box<dyn Fn(&native::UITextView)>>>,
    //pub on_change: Option<Box<dyn Fn(String)>>,
}
impl TextEditor {
    pub fn new(env: crate::EnvMarker) -> Self {
        Self {
            inner: native::UITextView::new(env),
            env,
            text: String::new(),
            event_fn: RefCell::new(None),
        }
    }
    pub fn with_event_fn(self: Self, event_fn: Box<dyn Fn(&native::UITextView)>) -> Self {
        *self.inner.event_fn().borrow_mut() = Some(event_fn);
        self
    }
}

#[cfg(target_os = "ios")]
mod native {
    use super::*;
    use objc2::rc::Retained;
    use objc2::runtime::ProtocolObject;
    use objc2::{define_class, msg_send, DeclaredClass, MainThreadOnly, Ivars};
    use objc2_foundation::{MainThreadMarker, NSObject, NSObjectProtocol, NSString};
    use objc2_ui_kit::{UIColor, UIResponder, UIScrollViewDelegate, UITextViewDelegate, UIView};
    use std::cell::RefCell;
    use winit::event_loop::EventLoopProxy;
    pub struct TextViewState {
        delegate: RefCell<Retained<TextFieldDelegate>>,
        //proxy: EventLoopProxy<GUIEvent>,
        pub event_fn: RefCell<Option<Box<dyn Fn(&UITextView)>>>,
        place_holder_text: RefCell<Option<String>>,
    }

    define_class!(
        #[unsafe(super(objc2_ui_kit::UITextView, UIView, UIResponder, NSObject))]
        #[thread_kind = MainThreadOnly]
        #[name = "FerrisUITextView"]
        //#[ivars = TextViewState]
        pub struct UITextView {
            delegate: RefCell<Retained<TextFieldDelegate>>,
            pub event_fn: RefCell<Option<Box<dyn Fn(&UITextView)>>>,
            place_holder_text: RefCell<Option<String>>,
        }

        impl UITextView { }
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
            fn did_change_selection(&self, _sender: &UITextView) {}
            #[unsafe(method(textViewDidBeginEditing:))]
            fn did_begin_editing(&self, sender: &UITextView) {
                sender.began_editing();
            }

            #[unsafe(method(textViewDidEndEditing:))]
            fn did_end_editing(&self, sender: &UITextView) {
                sender.ended_editing();
            }

            #[unsafe(method(textViewDidChange:))]
            fn did_change(&self, sender: &UITextView) {
                sender.text_changed();
            }
        }
    );

    impl UITextView {
        pub fn new(mtm: MainThreadMarker/*, proxy: EventLoopProxy<GUIEvent>*/) -> Retained<Self> {
            let delegate: Retained<TextFieldDelegate> =
                unsafe { objc2::msg_send![TextFieldDelegate::alloc(mtm), init] };
            let this = Self::alloc(mtm).set_ivars(Ivars::<Self> {
                delegate: RefCell::new(delegate),
                //proxy,
                event_fn: RefCell::new(None),
                place_holder_text: RefCell::new(None),
            });
            let this: Retained<UITextView> = unsafe { msg_send![super(this), init] };
            {
                let delegate = this.delegate().borrow();
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

        pub fn ended_editing(&self) {
            let place_holder_text = self
                .place_holder_text()
                .borrow()
                .clone()
                .unwrap_or_default();
            if !place_holder_text.is_empty() {
                self.setText(Some(&NSString::from_str(place_holder_text.as_str())));
                self.setTextColor(Some(&UIColor::grayColor()));
            }
        }

        fn text_changed(&self) {
            //let text = self.get_text();

            //let _ = self.ivars().proxy.send_event(GUIEvent::Text(text.clone()));

            if let Some(event_fn) = &*self.event_fn().borrow() {
                event_fn(self);
            }
        }

        pub fn with_place_holder_text(
            self: Retained<Self>,
            place_holder: String,
        ) -> Retained<Self> {
            *self.place_holder_text().borrow_mut() = Some(place_holder);
            self
        }
    }
}
impl View for TextEditor {
    fn raw_view(&self) -> Box<&crate::NativeView> {
        //self.ended_editing();
        Box::new(self.inner.as_ref())
    }
}
