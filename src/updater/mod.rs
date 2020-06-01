mod handler;
mod types;
pub use handler::PulseHandler;
pub use types::SinkInputData;

use std::rc::Rc;
use std::cell::RefCell;

use libpulse_binding as pulse;
use pulse::callbacks::ListResult;
use pulse::volume::{Volume, ChannelVolumes};

use crate::button;
use iced::slider;

pub fn update_sink_inputs(
    handler: &mut PulseHandler,
    sink_inputs: Rc<RefCell<Vec<SinkInputData>>>,
    sink_input_uis: &mut Vec<(slider::State, button::State)>
) {
    let new_sink_inputs: Rc<RefCell<Vec<SinkInputData>>> = Rc::new(RefCell::new(Vec::new()));
    let new_sink_inputs_ref = new_sink_inputs.clone();
    
    let op = handler.introspect.get_sink_input_info_list(
	move |x| match x {
	    ListResult::Item(item) if item.client.is_some() =>
		new_sink_inputs_ref.borrow_mut().push(item.into()),
	    _ => {}
	});
    handler.wait_for_operation(op);

    let mut sink_inputs = sink_inputs.borrow_mut();
    for i in 0..sink_inputs.len() {
	let new_sink_inputs = new_sink_inputs.borrow();
	
    	let find_result = new_sink_inputs
    	    .iter()
    	    .find(|new| new.id == sink_inputs.get(i).unwrap().id);
	
	if let Some(new) = find_result {
	    sink_inputs.get_mut(i).unwrap().volume = new.volume;
	    sink_inputs.get_mut(i).unwrap().mute   = new.mute;
	} else {
	    sink_inputs.remove(i);
	    sink_input_uis.remove(i);
	}
    }
    
    for new_sink_input in new_sink_inputs.borrow().iter() {
	if sink_inputs.iter().find(|old| old.id == new_sink_input.id).is_none() {
	    sink_inputs.push(new_sink_input.clone());
	    sink_input_uis.push((slider::State::new(), button::State::new()));
	}
    }
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
