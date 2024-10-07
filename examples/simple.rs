use ferris_ui::{App, GUIEvent, TextField, VStack, View};
use objc2::rc::Retained;
use objc2_foundation::{MainThreadMarker, NSString};
use objc2_ui_kit::{UIColor, UIEdgeInsets, UILabel, UISwitch, UITabBar, UIToolbar, UIView};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};

fn main() {
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
    let my_view = MyView {
        proxy: proxy.clone(),
    };

    let mut app = App::new(proxy, Box::new(my_view));
    let _ = event_loop.run_app(&mut app);
}

struct MyView {
    proxy: EventLoopProxy<GUIEvent>,
}
impl View for MyView {
    fn build(&self) -> Box<&UIView> {
        let mtm = MainThreadMarker::new().unwrap();
        let text_field = TextField::new(mtm, self.proxy.clone());
        unsafe {
            text_field.setText(Some(&NSString::from_str("THIS IS AN INPUT")));
            text_field.setBackgroundColor(Some(&UIColor::blueColor()));
            text_field.setLayoutMargins(UIEdgeInsets {
                top: 30.,
                left: 30.,
                bottom: 30.,
                right: 30.,
            });
        }
        let label = unsafe { UILabel::new(mtm) };

        unsafe {
            label.setText(Some(&NSString::from_str("baz")));
            label.setBackgroundColor(Some(&UIColor::redColor()));
            label.setLayoutMargins(UIEdgeInsets {
                top: 30.,
                left: 30.,
                bottom: 30.,
                right: 30.,
            });
        }

        /*
        let switch = unsafe { UISwitch::new(mtm) };
        let tabbar = unsafe { UITabBar::new(mtm) };
        let toolbar = unsafe { UIToolbar::new(mtm) };

        unsafe {
            toolbar.setItems(None);
        }
        */
        let vstack = VStack::new(
            mtm,
            None,
            vec![
                //Box::new(switch.clone()),
                Box::new(text_field),
                Box::new(label.clone()),
                //Box::new(toolbar),
                //Box::new(tabbar),
            ],
        );
        todo!();
        //Box::new(&vstack.build())
    }
}
