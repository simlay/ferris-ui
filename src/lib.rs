#![feature(core_intrinsics)]
#![feature(arbitrary_self_types)]
mod app;
pub mod screenshot;
mod text;
mod text_field;
mod text_view;
mod vstack;
mod navigator;

#[cfg(test)]
mod test;
#[cfg(test)]
pub use test::{TestResult};

pub use winit;
cfg_if::cfg_if! {
    if #[cfg(target_os = "ios")] {
        pub type EnvMarker  = objc2::MainThreadMarker;
        pub type NativeView = objc2_ui_kit::UIView;
        pub type NativeBox<T> = objc2::rc::Retained<T>;
        pub use objc2;
        pub use objc2_ui_kit;
    } else if #[cfg(target_os = "android")] {
        pub type EnvMarker  = jni::JNIEnv;
        pub type NativeView = jni::objects::JValue;
        pub type NativeBox<T> = Box<T>;
    }
}


pub use app::App;
/*
mod image;
mod switch;
pub use image::{Image, ImageType};
pub use switch::Switch;
*/
pub use text_field::TextField;

pub use text::Text;
//pub use text_view::UITextView;
pub use text_view::TextEditor;
pub use vstack::VStack;

pub trait View {
    fn raw_view(&self) -> Box<&crate::NativeView>;
    fn kind(&self) -> String {
        "Default".into()
    }

    /*
    fn with_event_fn(self: crate::NativeBox<Self>, _event_fn: Box<dyn Fn(&Self)>) -> crate::NativeBox<Self>
    where
        Self: Sized,
    {
        self
    }
    fn with_background_color(self, color: Retained<UIColor>) -> Self
    where
        Self: Sized,
    {
        let ui_view = self.raw_view();
        ui_view.setBackgroundColor(Some(&color));

        self
    }
    */
}

impl<T: AsRef<crate::NativeView>> View for crate::NativeBox<T> {
    fn raw_view(&self) -> Box<&crate::NativeView> {
        Box::new(self.as_ref())
    }
}

#[derive(Debug)]
pub enum GUIEvent {
    Text(String),
    SwitchToggle(bool),
}

pub enum ViewTree {
    //Root(Box<dyn View>),
}
