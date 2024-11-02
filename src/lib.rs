mod text_field;
mod vstack;
mod app;
mod switch;
mod text;
pub use app::App;
pub use vstack::VStack;
pub use text_field::TextField;
pub use switch::Switch;
pub use text::Text;

use objc2::rc::Retained;
use objc2_ui_kit::{
    UIView,
    UIColor,
};

pub trait View {
    fn event(&mut self, event: GUIEvent) {
    }
    fn ui_view(&self) -> Box<&UIView>;
    fn set_background_color(self, color: Retained<UIColor>) -> Self where Self: Sized {
        let ui_view = self.ui_view();
        unsafe {
            ui_view.setBackgroundColor(Some(&color));
        }
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
