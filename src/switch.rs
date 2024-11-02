use std::cell::RefCell;
use objc2_ui_kit::{UIColor, UIControlEvents, UIEdgeInsets, UIResponder, UISwitch, UIToolbar, UIView};
use objc2::runtime::ProtocolObject;
use objc2::rc::Retained;
use objc2_foundation::{
    CGPoint, CGRect, CGSize, MainThreadMarker, NSObject, NSObjectProtocol,
};
use objc2::{declare_class, msg_send_id, mutability, ClassType, DeclaredClass };

use winit::event_loop::EventLoopProxy;
use crate::GUIEvent;
use log::debug;

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

        let this = mtm.alloc().set_ivars(SwitchState {
            proxy,
            event_fn,
        });
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
        unsafe {
            self.isOn()
        }
    }
}
