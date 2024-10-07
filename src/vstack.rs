use crate::View;
use objc2::rc::Retained;
use objc2_foundation::{CGPoint, CGRect, CGSize, MainThreadMarker, NSArray, NSString};
use objc2_ui_kit::{
    UIColor, UIEdgeInsets, UILabel, UILayoutConstraintAxis, UIStackView, UIStackViewAlignment,
    UIStackViewDistribution, UISwitch, UITabBar, UIToolbar, UIView,
};

pub struct VStack {
    children: Vec<Box<dyn View>>,
    stack_view: Retained<UIStackView>,
}

impl VStack {
    pub fn new(mtm: MainThreadMarker, frame: Option<CGRect>, children: Vec<Box<dyn View>>) -> Self {
        let stack_view = unsafe { UIStackView::new(mtm) };
        unsafe {
            stack_view.setAxis(UILayoutConstraintAxis::Vertical);
            stack_view.setAlignment(UIStackViewAlignment::Fill);
            stack_view.setDistribution(UIStackViewDistribution::FillEqually);
        };

        if let Some(frame) = frame {
            stack_view.setFrame(frame);
        }
        Self {
            children,
            stack_view,
        }
    }
}
impl View for VStack {
    fn build(&self) -> Box<&UIView> {
        for child in &self.children {
            let child = child.build();
            unsafe { self.stack_view.addArrangedSubview(child.as_ref()) };
        }
        Box::new(self.stack_view.as_ref())
    }
}
