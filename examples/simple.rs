use ferris_ui::{
    GUIEvent,
    App,
};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};

fn main () {
    let event_loop : EventLoop<GUIEvent> = EventLoop::with_user_event().build().unwrap();
    //let event_loop = EventLoop::new().unwrap();
    let proxy : EventLoopProxy<GUIEvent> = event_loop.create_proxy();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::new(proxy);
    let _ = event_loop.run_app(&mut app);
}
