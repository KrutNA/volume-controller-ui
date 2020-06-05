use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use pulse::context::{
    flags as context_flags, introspect::Introspector, Context, State as ContextState,
};
use pulse::mainloop::standard::{IterateResult, Mainloop};
use pulse::operation::{Operation, State as OpState};
use pulse::proplist::{properties, Proplist};

pub struct PulseHandler {
    pub mainloop: Rc<RefCell<Mainloop>>,
    pub context: Rc<RefCell<Context>>,
    pub introspect: Introspector,
}

impl PulseHandler {
    pub fn new() -> Option<Self> {
        let mut proplist = Proplist::new().expect("Unable to create proplist");
        proplist
            .set_str(properties::APPLICATION_NAME, "VolumeController")
            .unwrap();

        let mainloop = Rc::new(RefCell::new(
            Mainloop::new().expect("Failed to create mainloop"),
        ));

        let context = Rc::new(RefCell::new(
            Context::new_with_proplist(
                mainloop.borrow().deref(),
                "VolumeControllerContext",
                &proplist,
            )
            .expect("Failed to create context"),
        ));

        context
            .borrow_mut()
            .connect(None, context_flags::NOFLAGS, None)
            .expect("Failed to connect context");

        loop {
            match mainloop.borrow_mut().iterate(false) {
                IterateResult::Quit(_) | IterateResult::Err(_) => {
                    eprintln!("Iterate state was not success, quitting...");
                    return None;
                }
                IterateResult::Success(_) => {}
            }
            match context.borrow().get_state() {
                ContextState::Ready => {
                    break;
                }
                ContextState::Failed | ContextState::Terminated => {
                    eprintln!("Context state failed/terminated, quitting...");
                    return None;
                }
                _ => {}
            }
        }

        let introspect = context.borrow_mut().introspect();
        Some(Self {
            mainloop,
            context,
            introspect,
        })
    }

    pub fn wait_for_operation<T: ?Sized>(&mut self, operation: Operation<T>) {
        loop {
            match self.mainloop.borrow_mut().iterate(false) {
                IterateResult::Err(e) => eprintln!("{}", e),
                IterateResult::Success(_) => {}
                IterateResult::Quit(_) => {
                    eprintln!("Iterate state quit without an error");
                    return;
                }
            }
            match operation.get_state() {
                OpState::Done => {
                    break;
                }
                OpState::Running => {}
                OpState::Cancelled => {
                    eprintln!("Operation cancelled without an error");
                    return;
                }
            }
        }
    }
}

impl Drop for PulseHandler {
    fn drop(&mut self) {
        self.context.borrow_mut().disconnect();
        self.mainloop.borrow_mut().quit(pulse::def::Retval(0));
    }
}
