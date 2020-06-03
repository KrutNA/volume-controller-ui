use std::rc::Rc;
use std::cell::RefCell;

#[cfg(feature = "time")]
use std::time::SystemTime;

use crate::updater::{
    SinkInputData, SinkData, PulseHandler,
    update_sink_inputs, update_sink_input_volume_by_id, update_sink_input_mute_by_id,
    update_fetch_sink, update_sink_volume, update_sink_mute,
};

use crate::button::WgpuButton as Button;
use crate::button;

use iced::{
    slider, scrollable,
    Slider, Scrollable,
    Sandbox, Element, Container, Column, Length, Text, Row,
    Align, HorizontalAlignment, VerticalAlignment
};

const MAX_VOLUME: u32 = 65536;
const MAX_VOLUME_FLOAT: f32 = 65536.0;
const MUTE_BUTTON_SIZE: u16 = 100;
const PROCENT_STATUS_SIZE: u16 = 100;
const APPLICATION_NAME_SIZE: u16 = 200;
const APPLICATION_NAME: &'static str = "Volume Controller";
const SINK_NAME: &'static str = "VOLUME";

pub struct UserInterface {
    pulse_handler:     PulseHandler,
    scroll:            scrollable::State,
    mute_button_texts: (Text, Text),
    
    sink_input_uis:    Vec<(slider::State, button::State)>,
    sink_input_datas:  Rc<RefCell<Vec<SinkInputData>>>,

    sink_ui:           (slider::State, button::State),
    sink_data:         Rc<RefCell<SinkData>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SliderChanged(usize, u32, u32),
    MuteButtonPressed(u32, bool),
    SinkSliderChanged(u32),
    SinkMuteButtonPressed(bool),
}

impl UserInterface {
    fn update_data(&mut self) {
	#[cfg(debug_assertions)]
	println!("Log: Updating!");
	
	update_sink_inputs(&mut self.pulse_handler,
			   self.sink_input_datas.clone(),
			   &mut self.sink_input_uis);

	update_fetch_sink (&mut self.pulse_handler,
			   self.sink_data.clone());
    }
}

impl Sandbox for UserInterface {
    type Message = Message;

    fn new() -> Self {
	Self {
	    pulse_handler:     PulseHandler::new().unwrap(),
	    scroll:            scrollable::State::new(),
	    mute_button_texts: (Text::new("Mute")
				.width(Length::Fill)
				.vertical_alignment(VerticalAlignment::Center)
				.horizontal_alignment(HorizontalAlignment::Center),
				Text::new("Unmute")
				.width(Length::Fill)
				.vertical_alignment(VerticalAlignment::Center)
				.horizontal_alignment(HorizontalAlignment::Center),),

	    sink_input_uis:    Vec::new(),
	    sink_input_datas:  Rc::new(RefCell::new(Vec::new())),

	    sink_ui: (slider::State::new(),
		      button::State::new()),
	    sink_data: Rc::new(RefCell::new(SinkData::default())),
	}
    }

    fn title(&self) -> String {
	String::from(APPLICATION_NAME)
    }

    fn update(&mut self, message: Message) {
	match message {
	    Message::SliderChanged(index, id, volume) => {
		#[cfg(debug_assertions)]
		println!("Log: slider with index {} of {} changed to {}.", index, id, volume);
		
		self.sink_input_datas.borrow_mut()[index].volume = volume;
		update_sink_input_volume_by_id(&mut self.pulse_handler, id, volume);
	    }
	    Message::MuteButtonPressed(id, status) => {
		#[cfg(debug_assertions)]
		println!("Log: button of {} pressed with status to {}.", id, status);
		
		update_sink_input_mute_by_id(&mut self.pulse_handler, id, status);
	    }
	    Message::SinkSliderChanged(volume) => {
		#[cfg(debug_assertions)]
		println!("Log: volume slider changed to {}.", volume);

		update_sink_volume(&mut self.pulse_handler, volume);
	    }

	    Message::SinkMuteButtonPressed(status) => {
		#[cfg(debug_assertions)]
		println!("Log: volumebutton pressed with status to {}.", status);

		update_sink_mute(&mut self.pulse_handler, status);
	    }
	}
    }

    fn view(&mut self) -> Element<Message> {
	self.update_data();

	#[cfg(feature = "time")]
	let start = SystemTime::now();
	
	let mut scrollable = Scrollable::new(&mut self.scroll)
            .width(Length::Fill)
            .height(Length::Fill);

	let sink_row = {
	    let sink_data = self.sink_data.borrow();
	    let is_mute = sink_data.mute;

	    let text   = Text::new(SINK_NAME)
		.width(Length::from(APPLICATION_NAME_SIZE))
		.vertical_alignment(VerticalAlignment::Center)
		.horizontal_alignment(HorizontalAlignment::Right);

	    let slider = Slider::new(&mut self.sink_ui.0,
				     0.0..=MAX_VOLUME_FLOAT,
				     sink_data.volume as f32,
				     move |v| Message::SinkSliderChanged(v as u32));
	    let button = Button::new(&mut self.sink_ui.1,
				     if is_mute { self.mute_button_texts.0.clone() }
				     else       { self.mute_button_texts.1.clone() },
				     move || Message::SinkMuteButtonPressed(
					 !is_mute))
    		.width(Length::from(MUTE_BUTTON_SIZE))
    		.padding(10);

	    let status = Text::new(&format!("{}%", sink_data.volume * 100 / MAX_VOLUME))
        	.horizontal_alignment(HorizontalAlignment::Center)
        	.vertical_alignment(VerticalAlignment::Center)
        	.width(Length::from(PROCENT_STATUS_SIZE));
	    
	    Row::new()
    		.align_items(Align::Center)
		.spacing(10)
		.push(text)
		.push(slider)
    		.push(status)
    		.push(button)
	};
	scrollable = scrollable.push(sink_row);

	let sink_input_datas = self.sink_input_datas.borrow();
	for (index, sink_input_ui) in self.sink_input_uis.iter_mut().enumerate() {
	    
	    let id      = sink_input_datas[index].id;
	    let is_mute = sink_input_datas[index].mute;

	    let text   = Text  ::new(sink_input_datas[index].name.clone())
		.width(Length::from(APPLICATION_NAME_SIZE))
		.vertical_alignment(VerticalAlignment::Center)
		.horizontal_alignment(HorizontalAlignment::Right);

	    let slider = Slider::new(&mut sink_input_ui.0,
				     0.0..=MAX_VOLUME_FLOAT,
				     sink_input_datas[index].volume as f32,
				     move |v| Message::SliderChanged(index, id, v as u32));

	    let button = Button::new(&mut sink_input_ui.1,
				     if is_mute { self.mute_button_texts.0.clone() }
				     else       { self.mute_button_texts.1.clone() },
				     move || Message::MuteButtonPressed(id, !is_mute))
    		.width(Length::from(MUTE_BUTTON_SIZE))
    		.padding(10);

	    let row    = Row::new()
    		.spacing(10)
    		.align_items(Align::Center)
    		.push(text)
    		.push(slider)
    		.push(Text::new(&format!("{}%",
					 sink_input_datas[index].volume
					 * 100 / MAX_VOLUME))
        	      .horizontal_alignment(HorizontalAlignment::Center)
        	      .vertical_alignment(VerticalAlignment::Center)
        	      .width(Length::from(PROCENT_STATUS_SIZE)))
    		.push(button);

	    scrollable = scrollable.push(row);
	}
	
	let content = Column::new()
	    .spacing(20)
	    .padding(20)
	    .push(scrollable);

	#[cfg(feature = "time")]
	println!("Initialized for {} s.",
		 SystemTime::now().duration_since(start).unwrap().as_secs_f64());
	
	Container::new(content)
	    .width(Length::Fill)
	    .height(Length::Fill)
	    .center_x()
	    .center_y()
	    .into()
    }
    
}
