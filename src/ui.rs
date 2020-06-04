use std::rc::Rc;
use std::cell::RefCell;

#[cfg(feature = "time")]
use std::time::SystemTime;

use crate::updater::{
    SinkInputData, MainData, PulseHandler,
    update_sink_inputs, update_sink_input_volume_by_id, update_sink_input_mute_by_id,
    update_fetch_sink,   update_sink_volume,   update_sink_mute,
    update_fetch_source, update_source_volume, update_source_mute,
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
const SINK_NAME:        &'static str = "System Volume";
const SOURCE_NAME:      &'static str = "Microphone";

pub struct UserInterface {
    pulse_handler:     PulseHandler,
    scroll:            scrollable::State,

    sink_input_uis:    Vec<(slider::State, button::State)>,
    sink_input_datas:  Rc<RefCell<Vec<SinkInputData>>>,

    sink_ui:           (slider::State, button::State),
    sink_data:         Rc<RefCell<MainData>>,

    source_ui:         (slider::State, button::State),
    source_data:       Rc<RefCell<MainData>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SliderChanged(usize, u32, u32),
    MuteButtonPressed(u32, bool),
    SinkSliderChanged(u32),
    SinkMuteButtonPressed(bool),
    SourceSliderChanged(u32),
    SourceMuteButtonPressed(bool),
}

impl Sandbox for UserInterface {
    type Message = Message;

    fn new() -> Self {
	Self {
	    pulse_handler:     PulseHandler::new().unwrap(),
	    scroll:            scrollable::State::new(),

	    sink_input_uis:    Vec::new(),
	    sink_input_datas:  Rc::new(RefCell::new(Vec::new())),

	    sink_ui:           (slider::State::new(), button::State::new()),
	    sink_data:         Rc::new(RefCell::new(MainData::default())),

	    source_ui:         (slider::State::new(), button::State::new()),
	    source_data:       Rc::new(RefCell::new(MainData::default())),
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
		println!("Log: volume button pressed with status to {}.", status);

		update_sink_mute(&mut self.pulse_handler, status);
	    }
	    
	    Message::SourceSliderChanged(volume) => {
		#[cfg(debug_assertions)]
		println!("Log: volume slider changed to {}.", volume);

		update_source_volume(&mut self.pulse_handler, volume);
	    }
	    Message::SourceMuteButtonPressed(status) => {
		#[cfg(debug_assertions)]
		println!("Log: volume button pressed with status to {}.", status);

		update_source_mute(&mut self.pulse_handler, status);
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

	{
	    let data    = self.sink_data.borrow();
	    let (is_mute, volume) = (data.mute, data.volume);
	    
    	    let text    = Self::create_name(SINK_NAME);
	    
    	    let slider  = Slider::new(&mut self.sink_ui.0,
    				      0.0 ..= MAX_VOLUME_FLOAT,
    				      volume as f32,
    				      move |v| Message::SinkSliderChanged(v as u32));
	    
	    let button  = Button::new(&mut self.sink_ui.1,
    				      Self::create_status_button(is_mute),
    				      move || Message::SinkMuteButtonPressed(!is_mute))
    		.width(Length::from(MUTE_BUTTON_SIZE))
    		.padding(10);
	    
    	    let status  = Self::create_status(volume);
	    
    	    let row = Row::new()
    		.align_items(Align::Center)
    		.spacing(10)
    		.push(text)
    		.push(slider)
    		.push(status)
    		.push(button);
	    scrollable = scrollable.push(row);
	}

	{
	    let data    = self.source_data.borrow();
	    let (is_mute, volume) = (data.mute, data.volume);
	    
    	    let text    = UserInterface::create_name(SOURCE_NAME);
    	    let slider  = Slider::new(&mut self.source_ui.0,
    				      0.0 ..= MAX_VOLUME_FLOAT,
    				      volume as f32,
    				      move |v| Message::SourceSliderChanged(v as u32));
	    let button  = Button::new(&mut self.source_ui.1,
    				      Self::create_status_button(is_mute),
    				      move || Message::SourceMuteButtonPressed(!is_mute))
    		.width(Length::from(MUTE_BUTTON_SIZE))
    		.padding(10);
    	    let status  = Self::create_status(volume);
	    
    	    let row = Row::new()
    		.align_items(Align::Center)
    		.spacing(10)
    		.push(text)
    		.push(slider)
    		.push(status)
    		.push(button);
	    
	    scrollable = scrollable.push(row);
	}

	let datas = self.sink_input_datas.borrow();
	for (index, ui) in self.sink_input_uis.iter_mut().enumerate() {
	    let row = {
		let (id, is_mute, volume) = (datas[index].id, datas[index].mute, datas[index].volume);
		
    		let text    = UserInterface::create_name(&datas[index].name);
    		let slider  = Slider::new(&mut ui.0,
    					  0.0 ..= MAX_VOLUME_FLOAT,
    					  volume as f32,
    					  move |v| Message::SliderChanged(index, id, v as u32));
    		let button  = Button::new(&mut ui.1,
    					  Self::create_status_button(is_mute),
    					  move || Message::MuteButtonPressed(id, !is_mute))
    		    .width(Length::from(MUTE_BUTTON_SIZE))
    		    .padding(10);
    		let status  = Self::create_status(volume);
		
    		Row::new()
    		    .spacing(10)
    		    .align_items(Align::Center)
    		    .push(text)
    		    .push(slider)
    		    .push(status)
    		    .push(button)
	    };
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


impl UserInterface {
    fn update_data(&mut self) {
	#[cfg(debug_assertions)]
	println!("Log: Updating.");
	
	update_sink_inputs (&mut self.pulse_handler,
			    self.sink_input_datas.clone(),
			    &mut self.sink_input_uis);

	update_fetch_sink  (&mut self.pulse_handler,
			    self.sink_data.clone());
	
	update_fetch_source(&mut self.pulse_handler,
			    self.source_data.clone());
    }

    fn create_name(name: &str) -> Text {
	Text::new(name.clone())
    	    .width(Length::from(APPLICATION_NAME_SIZE))
    	    .vertical_alignment(VerticalAlignment::Center)
    	    .horizontal_alignment(HorizontalAlignment::Right)
    }

    fn create_status(volume: u32) -> Text {
	Text::new(&format!("{}%", volume * 100 / MAX_VOLUME))
    	    .horizontal_alignment(HorizontalAlignment::Center)
    	    .vertical_alignment(VerticalAlignment::Center)
    	    .width(Length::from(PROCENT_STATUS_SIZE))
    }

    fn create_status_button(is_mute: bool) -> Text {
	if is_mute {
	    Text::new("Mute")
		.width(Length::Fill)
		.vertical_alignment(VerticalAlignment::Center)
		.horizontal_alignment(HorizontalAlignment::Center)
	} else {
	    Text::new("Unmute")
		.width(Length::Fill)
		.vertical_alignment(VerticalAlignment::Center)
		.horizontal_alignment(HorizontalAlignment::Center)
	}
    }
}
