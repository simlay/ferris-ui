use objc2::rc::Retained;
use objc2::{ClassType, DeclaredClass, declare_class, msg_send_id, mutability};
use objc2_foundation::{MainThreadMarker, NSObject};
use objc2_ui_kit::{UIControlEvents, UIResponder, UISwitch, UIView};

use crate::{GUIEvent, View};
use log::debug;
use winit::event_loop::EventLoopProxy;

pub struct SwitchState {
    proxy: EventLoopProxy<GUIEvent>,
    event_fn: Option<Box<dyn Fn(&Switch)>>,
}

declare_class!(
    pub struct Switch;
    unsafe impl ClassType for Switch {
        #[inherits(UIView, UIResponder, NSObject)]
        type Super = UISwitch;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "FerrisUISwitch";
    }

    impl DeclaredClass for Switch {
        type Ivars = SwitchState;
    }

    unsafe impl Switch {
        #[method(toggle)]
        fn toggle(&self) {
            let is_on = self.is_on();
            if let Some(event_fn) = &self.ivars().event_fn {
                event_fn(self);
            }

            let _ = self.ivars().proxy.send_event(GUIEvent::SwitchToggle(is_on));
            debug!("SWITCH TOGGLED!: {is_on}");
        }
    }
);

impl Switch {
    pub fn new(
        proxy: EventLoopProxy<GUIEvent>,
        event_fn: Option<Box<dyn Fn(&Self)>>,
    ) -> Retained<Self> {
        let mtm = MainThreadMarker::new().unwrap();

        let this = mtm.alloc().set_ivars(SwitchState { proxy, event_fn });
        let this: Retained<Self> = unsafe { msg_send_id![super(this), init] };

        unsafe {
            this.addTarget_action_forControlEvents(
                Some(&this),
                objc2::sel!(toggle),
                UIControlEvents::UIControlEventValueChanged,
            );
        }

        this
    }
    pub fn is_on(&self) -> bool {
        unsafe { self.isOn() }
    }
}

impl View for Switch {
    fn ui_view(&self) -> Box<&UIView> {
        Box::new(self.as_ref())
    }
    #[cfg(feature = "nightly")]
    fn set_event_fn(self: Retained<Self>, event_fn: Box<dyn Fn(&Self)>) -> Retained<Self> {
        let ivars = self.ivars();
        Self::new(ivars.proxy.clone(), Some(event_fn))
    }
}
