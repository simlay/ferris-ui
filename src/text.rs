use crate::View;

#[derive(Clone)]
pub struct Text {
    pub text: String,
    pub font_size: Option<f32>,
    // On android this will be the JNIenv
    env: crate::EnvMarker,
    inner: crate::NativeBox<native::UIText>,
}
impl Text {
    pub fn new<T: Into<String>>(text: T, env: crate::EnvMarker) -> Self {
        Self {
            text: text.into(),
            font_size: None,
            env,
            inner: native::UIText::new(env),
        }
    }
    #[cfg(target_os = "ios")]
    pub fn set_text<T: Into<String>>(&self, new_text: T) {
        self.inner.setText(Some(&objc2_foundation::NSString::from_str(&new_text.into())));
    }
}

mod native {
    use objc2::rc::{PartialInit, Retained};
    use objc2::{MainThreadMarker, MainThreadOnly, define_class, msg_send};
    use objc2_foundation::NSObject;
    use objc2_ui_kit::{UILabel, UIView};

    define_class!(
        #[unsafe(super(UILabel, UIView, NSObject))]
        #[thread_kind = MainThreadOnly]
        #[name = "FerrisUILabel"]
        pub struct UIText;

        impl UIText { }
    );

    impl UIText {
        pub fn new(mtm: MainThreadMarker) -> Retained<Self> {
            let this: PartialInit<Self> = mtm.alloc().set_ivars(());
            let this: Retained<Self> = unsafe { msg_send![super(this), init] };
            this
        }
    }

}
impl View for Text {
    fn raw_view(&self) -> Box<&crate::NativeView> {
        Box::new(self.inner.as_ref())
    }
}
