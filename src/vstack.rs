use crate::View;
use objc2::rc::Retained;
use objc2_foundation::MainThreadMarker;
use objc2_ui_kit::{
    UILayoutConstraintAxis,
    UIStackView, UIStackViewAlignment, UIStackViewDistribution, UIView,
    UIColor, UIEdgeInsets,
};


type VStackChildren = Vec<Box<dyn View>>;
pub struct VStack {
    children: VStackChildren,
    stack_view: Retained<UIStackView>,
}

impl VStack {
    pub fn new(mtm: MainThreadMarker, children: VStackChildren) -> Self {
        //let stack_view = unsafe { UIStackView::new(mtm) };
        let stack_view = unsafe { UIStackView::new(mtm) };
        unsafe {
            stack_view.setAxis(UILayoutConstraintAxis::Vertical);
            stack_view.setAlignment(UIStackViewAlignment::Fill);
            stack_view.setDistribution(UIStackViewDistribution::FillEqually);
            //stack_view.setSpacing(10.);
            stack_view.setBackgroundColor(Some(&*UIColor::whiteColor()));
            stack_view.setTranslatesAutoresizingMaskIntoConstraints(true);
            stack_view.setLayoutMargins(
                UIEdgeInsets {
                    top: 20.,
                    left: 20.,
                    bottom: 20.,
                    right: 20.,
                }
            )
        };

        Self {
            children,
            stack_view,
        }
    }
}
impl View for VStack {
    fn ui_view(&self) -> Box<&UIView> {
        for child in &self.children {
            let child = child.ui_view();

            // TODO: Make this debug/feature flagged.
            let layer = child.layer();
            layer.setBorderWidth(1.);
            unsafe {
                layer.setBorderColor(Some(&UIColor::greenColor().CGColor()));
            }

            unsafe { self.stack_view.addArrangedSubview(child.as_ref()) };
        }
        Box::new(self.stack_view.as_ref())
    }
}
