use crate::View;
use objc2::rc::Retained;
use objc2_foundation::MainThreadMarker;
use objc2_ui_kit::{
    UIColor, UIEdgeInsets, UILayoutConstraintAxis, UIStackView, UIStackViewDistribution, UIView,
};

type VStackChildren = Vec<Box<dyn View>>;
pub struct VStack {
    children: VStackChildren,
    stack_view: Retained<UIStackView>,
}

impl VStack {
    pub fn new(mtm: MainThreadMarker, children: VStackChildren) -> Self {
        let stack_view = UIStackView::new(mtm);
        stack_view.setAxis(UILayoutConstraintAxis::Vertical);
        //stack_view.setAlignment(UIStackViewAlignment::Center);
        stack_view.setDistribution(UIStackViewDistribution::FillEqually);
        stack_view.setSpacing(10.);
        //stack_view.setTranslatesAutoresizingMaskIntoConstraints(true);
        stack_view.setLayoutMarginsRelativeArrangement(true);
        stack_view.setLayoutMargins(UIEdgeInsets {
            top: 100.,
            left: 50.,
            bottom: 10.,
            right: 50.,
        });
            /*
            */
        let layer = stack_view.layer();
        layer.setBorderWidth(1.);
        println!("Vstack BOUNDS: {:?}", layer.bounds());
        unsafe {
            layer.setBorderColor(Some(&UIColor::redColor().CGColor()));
        }

        Self {
            children,
            stack_view,
        }
    }
}

impl View for VStack {
    fn raw_view(&self) -> Box<&UIView> {
        for child in &self.children {
            let ui_child = child.raw_view();

            // TODO: Make this debug/feature flagged.
            let layer = ui_child.layer();
            layer.setBorderWidth(1.);
            unsafe {
                layer.setBorderColor(Some(&UIColor::greenColor().CGColor()));
            }

            self.stack_view.addArrangedSubview(ui_child.as_ref());
            println!("CHILD {} BOUNDS{:?}", child.kind(), ui_child.bounds());
        }
        Box::new(self.stack_view.as_ref())
    }
}
