use ferris_ui::{App, GUIEvent, Switch, Text, TextField, VStack, View};
use objc2::rc::Retained;
use objc2_foundation::{MainThreadMarker, NSString};
use objc2_ui_kit::{UIColor, UIEdgeInsets, UILabel, UISwitch, UITabBar, UIToolbar, UIView};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};

fn main() {
    env_logger::init();
    let event_loop: EventLoop<GUIEvent> = EventLoop::with_user_event().build().unwrap();
    //let event_loop = EventLoop::new().unwrap();
    let proxy: EventLoopProxy<GUIEvent> = event_loop.create_proxy();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::new(proxy, Box::new(MyView::new));
    let _ = event_loop.run_app(&mut app);
}

struct MyView {
    proxy: EventLoopProxy<GUIEvent>,
    vstack: VStack,
}

impl MyView {
    pub fn new(proxy: EventLoopProxy<GUIEvent>) -> Box<dyn View> {
        let switch = Switch::new(proxy.clone(), None)
            .set_event_fn(Box::new(move |switch| {
                let is_on = switch.is_on();
                if is_on {
                    unsafe { switch.setBackgroundColor(Some(&UIColor::blueColor())) }
                } else {
                    unsafe { switch.setBackgroundColor(Some(&UIColor::cyanColor())) }
                }
            }))
            .set_background_color(unsafe { UIColor::cyanColor() });

        let label = Text::new();
        label.set_text(format!("Switch state: {}", switch.is_on()));

        unsafe {
            label.setBackgroundColor(Some(&UIColor::redColor()));
        }
        let cloned_label = label.clone();

        let text_field =
            TextField::new(proxy.clone(), None).set_event_fn(Box::new(move |text_field| {
                let new_text = unsafe { text_field.text() }.to_string();
                let text = format!("Current text: {new_text}");
                cloned_label.set_text(text);
            }));

        unsafe {
            text_field.setBackgroundColor(Some(&UIColor::blueColor()));
        }

        let vstack = VStack::new(vec![
            Box::new(text_field),
            Box::new(label.clone()),
            Box::new(switch.clone()),
        ]);
        Box::new(Self { proxy, vstack })
    }
}
impl View for MyView {
    fn ui_view(&self) -> Box<&UIView> {
        Box::new(&self.vstack.ui_view())
    }
}
