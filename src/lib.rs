mod text_field;
use objc2::rc::Retained;
use objc2_foundation::{CGPoint, CGRect, CGSize, MainThreadMarker, NSArray, NSString};
use objc2_ui_kit::{
    UIColor, UILabel, UILayoutConstraintAxis, UIStackView, UIStackViewAlignment, UIStackViewDistribution, UISwitch, UITabBar, UIToolbar, UIView
};
pub use text_field::TextField;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoopProxy};
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winit::window::{Window, WindowId};

#[derive(Debug)]
pub enum GUIEvent {
    Text(String),
}

pub trait View {
    fn build(&self) -> Box<&UIView>;
}

impl<T: AsRef<UIView>> View for Retained<T> {
    fn build(&self) -> Box<&UIView> {
        Box::new(self.as_ref().as_ref())
    }
}

struct VStack {
    children: Vec<Box<dyn View>>,
    stack_view: Retained<UIStackView>,
}

impl VStack {
    pub fn new(mtm: MainThreadMarker, frame: Option<CGRect>, children: Vec<Box<dyn View>>) -> Self {
        let stack_view = unsafe { UIStackView::new(mtm) };
        unsafe { stack_view.setAxis(UILayoutConstraintAxis::Vertical) };
        unsafe { stack_view.setAlignment(UIStackViewAlignment::Fill) };
        unsafe { stack_view.setDistribution(UIStackViewDistribution::FillEqually) };

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

pub enum ViewTree {
    //Root(Box<dyn View>),
}

pub struct App {
    window: Option<Window>,
    root_view: Option<Retained<UIView>>,
    proxy: EventLoopProxy<GUIEvent>,
    ui_label: Option<Retained<UILabel>>,
}

impl App {
    pub fn new(proxy: EventLoopProxy<GUIEvent>) -> Self {
        Self {
            window: None,
            root_view: None,
            ui_label: None,
            proxy,
        }
    }
}

impl ApplicationHandler<GUIEvent> for App {
    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: GUIEvent) {
        println!("NEW EVENT: {event:?}");
        if let GUIEvent::Text(text) = event {
            unsafe {
                self.ui_label
                    .as_ref()
                    .unwrap()
                    .setText(Some(&NSString::from_str(text.as_str())))
            };
        }
    }
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();
        let mtm = MainThreadMarker::new().unwrap();
        let text_field = TextField::new(mtm, self.proxy.clone());
        let label = unsafe { UILabel::new(mtm) };

        unsafe { label.setText(Some(&NSString::from_str("baz"))) };
        unsafe { label.setBackgroundColor(Some(&UIColor::redColor())) };

        let switch = unsafe { UISwitch::new(mtm) };
        let tabbar = unsafe { UITabBar::new(mtm) };
        let toolbar = unsafe { UIToolbar::new(mtm) };

        unsafe {
            toolbar.setItems(
                None
            );
        }

        if let Ok(handle) = window.window_handle() {
            if let RawWindowHandle::UiKit(handle) = handle.as_raw() {
                let ui_view = handle.ui_view.as_ptr();
                let ui_view: Retained<UIView> =
                    unsafe { Retained::retain(ui_view.cast()) }.unwrap();
                let root_frame = ui_view.frame();
                let vstack = VStack::new(
                    mtm,
                    Some(root_frame),
                    vec![
                        Box::new(switch.clone()),
                        Box::new(label.clone()),
                        Box::new(text_field),
                        Box::new(toolbar),
                        Box::new(tabbar),
                    ],
                );
                unsafe { ui_view.addSubview(vstack.build().as_ref()) };

                ui_view.setBackgroundColor(Some(unsafe { &UIColor::greenColor() }));
                self.root_view = Some(ui_view);
            }
        }

        self.ui_label = Some(label);
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            e => {
                println!("{e:#?}");
            }
        }
    }
}
