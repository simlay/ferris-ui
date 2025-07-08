#![cfg_attr(feature = "nightly-debug", feature(core_intrinsics))]
#![cfg_attr(feature = "nightly", feature(arbitrary_self_types))]
mod app;
mod screenshot;
mod switch;
mod text;
mod text_view;
mod vstack;
mod image;
mod text_field;

pub use app::App;
pub use switch::Switch;
pub use text::Text;
pub use text_view::TextView;
pub use vstack::VStack;
pub use image::{
    Image,
    ImageType,
};
pub use text_field::TextField;

use objc2::rc::Retained;
use objc2_ui_kit::{UIColor, UIView, UIEdgeInsets};

pub trait View {
    fn event(&mut self, _event: GUIEvent) {}
    fn ui_view(&self) -> Box<&UIView>;
    fn with_background_color(self, color: Retained<UIColor>) -> Self
    where
        Self: Sized,
    {
        let ui_view = self.ui_view();
        ui_view.setBackgroundColor(Some(&color));

        self
    }
    #[cfg(feature = "nightly")]
    fn with_event_fn(self: Retained<Self>, _event_fn: Box<dyn Fn(&Self)>) -> Retained<Self>
    where
        Self: Sized,
    {
        self
    }
}

impl<T: AsRef<UIView>> View for Retained<T> {
    fn ui_view(&self) -> Box<&UIView> {
        Box::new(self.as_ref().as_ref())
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
