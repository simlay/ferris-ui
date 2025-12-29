use ferris_ui::{
    NativeView,
    EnvMarker,
};
use ferris_ui::winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};
use ferris_ui::{App, GUIEvent, Text, TextEditor, VStack, View};
fn create_observer(
    center: &objc2_foundation::NSNotificationCenter,
    name: &objc2_foundation::NSNotificationName,
    handler: impl Fn(&objc2_foundation::NSNotification) + 'static,
) -> objc2::rc::Retained<objc2::runtime::ProtocolObject<dyn objc2_foundation::NSObjectProtocol>> {
    let block = block2::RcBlock::new(move |notification: std::ptr::NonNull<objc2_foundation::NSNotification>| {
        handler(unsafe { notification.as_ref() });
    });

    unsafe { center.addObserverForName_object_queue_usingBlock(Some(name), None, None, &block) }
}

fn main() {
    if std::env::var("LLVM_PROFILE_FILE").is_ok() {
        let _will_terminate_observer = create_observer(
            &objc2_foundation::NSNotificationCenter::defaultCenter(),
            unsafe { objc2_ui_kit::UIApplicationDidEnterBackgroundNotification },
            move |_notification| {
                println!("APP IS CLOSING");
                unsafe extern "C" {
                    safe fn __llvm_profile_write_file() -> std::ffi::c_int;
                }
                let res = __llvm_profile_write_file();
                assert_eq!(res, 0);
            },
        );
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
        let mtm = EnvMarker::new().expect("Failed to get main thread marker");
        //let label = UIText::new(mtm).with_text("Current text : ");
        let label : Text = Text::new("foobar", mtm);

        /*
        let switch_label_cloned = label.clone();

        let switch = Switch::new(mtm, proxy.clone())
            .with_event_fn(Box::new(move |switch| {
                let is_on = switch.is_on();
                let new_color = if is_on {
                    &UIColor::purpleColor()
                } else {
                    &UIColor::cyanColor()
                };
                switch.setBackgroundColor(Some(new_color));
                switch_label_cloned.set_text(format!("Switch is {}", is_on));
            }))
                ;
        */

        let label_for_text_view = label.clone();
        let text_view = TextEditor::new(mtm/*, proxy.clone()*/)
            .with_event_fn(Box::new(move |text_field| {
                let new_text = text_field.get_text();
                let text = format!("{new_text}");
                if new_text == "exit" {
                    std::process::exit(0);
                }
                label_for_text_view.set_text(text);
            }))
            /*
            .with_place_holder_text("PLACE HOLDER TEXT".into())
            */
                ;

        /*
        let label_for_text_field = label.clone();
        let text_field = TextField::new(mtm, proxy.clone())
            .with_event_fn(Box::new(move |text_field| {
                let new_text = text_field.get_text().unwrap_or_default();
                let text = format!("Current text: {new_text}");
                label_for_text_field.set_text(text);
            }))
            .with_place_holder_text("PLACE HOLDER TEXT".into())
                ;

        let image = Image::new(mtm, ImageType::SystemIcon("clock".into()));
        */

        let vstack = VStack::new(
            mtm,
            vec![
                Box::new(label.clone()),
                /*
                Box::new(image.clone()),
                Box::new(switch.clone()),
                */
                //Box::new(text_field),
                Box::new(text_view),
            ],
        )
            ;
        //.with_background_color(UIColor::whiteColor());

        Box::new(Self { proxy, vstack })
    }
}
impl View for MyView {
    fn raw_view(&self) -> Box<&NativeView> {
        Box::new(&self.vstack.raw_view())
    }
}
