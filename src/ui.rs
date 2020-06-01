use std::rc::Rc;
use std::cell::RefCell;

use crate::updater::{
    SinkInputData, PulseHandler,
    update_sink_inputs,
    update_sink_input_volume_by_id,
    update_sink_input_mute_by_id,
};

use crate::button::WgpuButton as Button;
use crate::button;

use iced::{
    slider, scrollable,
    Slider, Scrollable,
    Sandbox, Element, Container, Column, Length, Text, Row,
    Align, HorizontalAlignment, VerticalAlignment
};

const MUTE_BUTTON_SIZE: u16 = 200;
const APPLICAATION_NAME_SIZE: u16 = 200;

pub struct UserInterface {
    pulse_handler: PulseHandler,
    sink_input_uis: Vec<(slider::State, button::State)>,
    sink_input_data: Rc<RefCell<Vec<SinkInputData>>>,
    // sliders: Rc<RefCell<Vec<(Data, slider::State, button::State)>>>,
    scroll:  scrollable::State,
    mute_button_texts: (Text, Text),
}

#[derive(Debug, Clone)]
pub enum Message {
    SliderChanged(usize, u32, u32),
    MuteButtonPressed(u32, bool),
}

impl UserInterface {
    fn update_data(&mut self) {
	println!("Log: Updating!");
	
	self.sink_input_data = Rc::new(RefCell::new(Vec::new()));
	update_sink_inputs(&mut self.pulse_handler, self.sink_input_data.clone());
	
	self.sink_input_uis = Vec::new();
	for _ in 0..self.sink_input_data.borrow().len() {
	    self.sink_input_uis.push((slider::State::new(),
				      button::State::new()));
	}
    }
}

impl Sandbox for UserInterface {
    type Message = Message;

    fn new() -> Self {
	Self {
	    pulse_handler:     PulseHandler::new().unwrap(),
	    sink_input_uis:    Vec::new(),
	    sink_input_data:   Rc::new(RefCell::new(Vec::new())),
	    scroll: scrollable::State::new(),
	    mute_button_texts: (Text::new("Mute")
				.width(Length::Fill)
				.vertical_alignment(VerticalAlignment::Center)
				.horizontal_alignment(HorizontalAlignment::Center),
				Text::new("Unmute")
				.width(Length::Fill)
				.vertical_alignment(VerticalAlignment::Center)
				.horizontal_alignment(HorizontalAlignment::Center),),
	}
    }

    fn title(&self) -> String {
	String::from("Volume Level Configuration")
    }

    fn update(&mut self, message: Message) {
	match message {
	    Message::SliderChanged(index, id, volume) => {
		println!("Log: slider moved with index: {}, id: {}, value: {}!", index, id, volume);
		
		self.sink_input_data.borrow_mut().get_mut(index).unwrap().volume = volume;
		
		update_sink_input_volume_by_id(&mut self.pulse_handler, id, volume)
	    }
	    Message::MuteButtonPressed(id, status) => {
		update_sink_input_mute_by_id(&mut self.pulse_handler, id, status)
	    }
	}
    }

    fn view(&mut self) -> Element<Message> {
	self.update_data();

	let mut scrollable = Scrollable::new(&mut self.scroll)
            .width(Length::Fill)
            .height(Length::Fill);

	let sink_input_data_ref = self.sink_input_data.clone();
	
	for (index, sink_input_uis) in self.sink_input_uis.iter_mut().enumerate() {
	    
	    let id      = sink_input_data_ref.borrow().get(index).unwrap().id;
	    let is_mute = sink_input_data_ref.borrow().get(index).unwrap().mute;

	    let text   = Text  ::new(sink_input_data_ref.borrow().get(index).unwrap().name.clone())
		.width(Length::from(APPLICAATION_NAME_SIZE))
		.vertical_alignment(VerticalAlignment::Center)
		.horizontal_alignment(HorizontalAlignment::Right);
	    
	    let slider = Slider::new(&mut sink_input_uis.0,
				     0.0..=65536.0,
				     sink_input_data_ref.borrow().get(index).unwrap().volume as f32,
				     move |v| Message::SliderChanged(index, id, v as u32));

	    let m_bttn = Button::new(&mut sink_input_uis.1,
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
    		.push(m_bttn);

	    scrollable = scrollable.push(row);
	}
	
	// for (i, data) in self.sliders.iter_mut().enumerate() {
	//     let (id, is_muted, name) = (data.0.id, data.0.mute, data.0.name.to_owned());
	//     let slider = Slider::new(&mut data.1,
	// 			     0.0..=65536.0,
	// 			     data.0.volume as f32,
	// 			     move |v| Message::SliderChanged(i, id, v as usize));

	//     let mute_button = Button::new(&mut data.2,
	//     				  Text::new(if is_muted { "Unmute" } else { "Mute" }),
	// 				  move || Message::MuteButtonPressed(id, is_muted))
	//     	.width(Length::from(100))
    	//     	.padding(10);

	//     let row = Row::new()
    	//     	.spacing(10)
        //         .align_items(Align::Center)
    	//     	.push(Text::new(name).width(Length::from(200)))
    	//     	.push(slider)
    	//     	.push(mute_button);
	    
	//     scrollable = scrollable.push(row);
	// }
	
	let content = Column::new()
	    .spacing(20)
	    .padding(20)
	    // .push(Text::new(self.text.to_owned()))
	    .push(scrollable);
	
	Container::new(content)
	    .width(Length::Fill)
	    .height(Length::Fill)
	    .center_x()
	    .center_y()
	    .into()
    }
    
}
