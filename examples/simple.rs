use ferris_ui::objc2::MainThreadMarker;
use ferris_ui::objc2_ui_kit::{UIColor, UIView};
use ferris_ui::winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};
use ferris_ui::{App, GUIEvent, Image, ImageType, Switch, Text, TextField, TextView, VStack, View};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1) == Some(&"--record".to_string()) {
        println!("ARGS: {:?}", args);
    }
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
        let mtm = MainThreadMarker::new().unwrap();
        let label = Text::new(mtm).with_text("Current text : ");

        let switch_label_cloned = label.clone();

        let switch = Switch::new(mtm, proxy.clone())
            .with_event_fn(Box::new(move |switch| {
                let is_on = switch.is_on();
                let new_color = if is_on {
                    unsafe { &UIColor::purpleColor() }
                } else {
                    unsafe { &UIColor::cyanColor() }
                };
                switch.setBackgroundColor(Some(new_color));
                switch_label_cloned.set_text(format!("Switch is {}", is_on));
            }))
                ;

        let label_for_text_view = label.clone();
        let label_for_text_field = label.clone();
        let text_view = TextView::new(mtm, proxy.clone())
            .with_event_fn(Box::new(move |text_field| {
                let new_text = text_field.get_text();
                let text = format!("Current text: {new_text}");
                label_for_text_view.set_text(text);
            }))
            .with_place_holder_text("PLACE HOLDER TEXT".into());

        let text_field = TextField::new(mtm, proxy.clone())
            .with_event_fn(Box::new(move |text_field| {
                let new_text = text_field.get_text().unwrap_or_default();
                let text = format!("Current text: {new_text}");
                label_for_text_field.set_text(text);
            }))
                ;

        let image = Image::new(mtm, ImageType::SystemIcon("clock".into()));

        let vstack = VStack::new(
            mtm,
            vec![
                Box::new(label.clone()),
                Box::new(image.clone()),
                Box::new(switch.clone()),
                Box::new(text_field),
                //Box::new(text_view),
            ],
        )
        .with_background_color(unsafe { UIColor::whiteColor() });

        Box::new(Self { proxy, vstack })
    }
}
impl View for MyView {
    fn ui_view(&self) -> Box<&UIView> {
        Box::new(&self.vstack.ui_view())
    }
}
