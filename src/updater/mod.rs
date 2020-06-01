mod handler;
mod types;
pub use handler::PulseHandler;
pub use types::SinkInputData;

use std::rc::Rc;
use std::cell::RefCell;

use libpulse_binding as pulse;
use pulse::callbacks::ListResult;
use pulse::volume::{Volume, ChannelVolumes};

pub fn update_sink_inputs(
    handler: &mut PulseHandler,
    sink_inputs: Rc<RefCell<Vec<SinkInputData>>>,
) {
    let sink_inputs_ref = sink_inputs.clone();
    
    let op = handler.introspect.get_sink_input_info_list(
	move |x| match x {
	    ListResult::Item(item) if item.client.is_some() =>
		sink_inputs_ref.borrow_mut().push(item.into()),
	    _ => {}
	});
    handler.wait_for_operation(op);
}

pub fn update_sink_input_volume_by_id(
    handler: &mut PulseHandler,
    id: u32,
    volume: u32,
) {
    let mut channel_volumes = ChannelVolumes::default();
    channel_volumes.set(2, Volume(volume));
    
    let op = handler.introspect.set_sink_input_volume(id, &channel_volumes, None);

    handler.wait_for_operation(op);
}

pub fn update_sink_input_mute_by_id(
    handler: &mut PulseHandler,
    id: u32,
    status: bool,
) {
    let op = handler.introspect.set_sink_input_mute(id, status, None);
    
    handler.wait_for_operation(op);
}
