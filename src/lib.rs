mod text_field;
mod vstack;
mod app;
pub use app::App;
pub use vstack::VStack;
pub use text_field::TextField;

use objc2::rc::Retained;
use objc2_ui_kit::UIView;

pub trait View {
    fn build(&self) -> Box<&UIView>;
    fn view_id(&self) -> u32 {
        todo!()
    }
}

impl<T: AsRef<UIView>> View for Retained<T> {
    fn build(&self) -> Box<&UIView> {
        Box::new(self.as_ref().as_ref())
    }
}

#[derive(Debug)]
pub enum GUIEvent {
    Text(String),
}

pub enum ViewTree {
    //Root(Box<dyn View>),
}
