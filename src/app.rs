use objc2::rc::Retained;
use objc2_foundation::{MainThreadMarker, NSString};
use objc2_ui_kit::{
    UIColor, UIEdgeInsets, UILabel, UIView,
    UISwitch, UITabBar, UIToolbar,
};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoopProxy};
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winit::window::{Window, WindowId};
use crate::{
    View,
    TextField,
    VStack,
    GUIEvent,
};

pub struct App {
    window: Option<Window>,
    root_ui_view: Option<Retained<UIView>>,
    root_view: Box<dyn View>,
    proxy: EventLoopProxy<GUIEvent>,
    ui_label: Option<Retained<UILabel>>,
}

impl App {
    pub fn new(proxy: EventLoopProxy<GUIEvent>, root_view: Box<dyn View> ) -> Self {
        Self {
            window: None,
            root_ui_view: None,
            root_view,
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


        if let Ok(handle) = window.window_handle() {
            if let RawWindowHandle::UiKit(handle) = handle.as_raw() {
                let ui_view = handle.ui_view.as_ptr();
                let ui_view: Retained<UIView> =
                    unsafe { Retained::retain(ui_view.cast()) }.unwrap();
                let root_frame = ui_view.frame();
                let root_ui_view = self.root_view.build();
                root_ui_view.setFrame(root_frame);
                unsafe { ui_view.addSubview(root_ui_view.as_ref()) };

                ui_view.setBackgroundColor(Some(unsafe { &UIColor::greenColor() }));
                self.root_ui_view = Some(ui_view);
            }
        }

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
