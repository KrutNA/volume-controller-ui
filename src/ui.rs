use crate::updater::{
    Data,
    update as update_data,
    call as call_volume_edit,
    // mute as call_muting
};

use iced::{
    slider, scrollable, // button,
    Slider, Scrollable, // Button,
    Sandbox, Element, Container, Column, Length, Text, Row, Align,
};

#[derive(Default)]
pub struct UserInterface {
    sliders: Vec<(Data, slider::State)>,
    // sliders: Vec<(Data, slider::State, button::State)>,
    // text:    String,
    scroll:  scrollable::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    SliderChanged(usize, usize, usize),
    // MuteButtonPress(usize, bool),
}

impl UserInterface {    
    fn init_data() -> Vec<(
	Data,
	slider::State,
	// button::State,
    )> {
	update_data()
	    .into_iter()
	    .map(|v| { (v, slider::State::new()) })
	    // .map(|v| { (v, slider::State::new(), button::State::new()) })
	    .collect()
    }
}

impl Sandbox for UserInterface {
    type Message = Message;

    fn new() -> Self {
	Self {
	    sliders: Self::init_data(),
	    // text:    String::from("Debug"),
	    scroll:  scrollable::State::new(),
	}
    }

    fn title(&self) -> String {
	String::from("Volume Level Configuration")
    }

    fn update(&mut self, message: Message) {
	match message {
	    Message::SliderChanged(_index, id, value) => {
		call_volume_edit(id, value);
	    },
	    // Message::MuteButtonPress(id, status) => {
	    // 	call_muting(id, status);
	    // }
	}
    }

    fn view(&mut self) -> Element<Message> {

	let mut scrollable = Scrollable::new(&mut self.scroll)
            .width(Length::Fill)
            .height(Length::Units(100));

	self.sliders = Self::init_data();

	for (i, data) in self.sliders.iter_mut().enumerate() {
	    let (id, _is_muted, name) = (data.0.id, data.0.mute, data.0.name.to_owned());
	    let slider = Slider::new(&mut data.1,
				     0.0..=65536.0,
				     data.0.volume as f32,
				     move |v| Message::SliderChanged(i, id, v as usize));

	    // let mute_button = Button::new(&mut data.2,
	    // 				  Text::new(String::from(
	    // 				      if is_muted { "Unmute" } else { "Mute" })))
    	    // 	.padding(10)
    	    // 	.on_press(move || Message::MuteButtonPress(id, is_muted));
	    
	    let row = Row::new()
    	    	.spacing(10)
                .align_items(Align::Center)
    	    	.push(Text::new(name).width(Length::from(100)))
    	    	// .push(mute_button);
    	    	.push(slider);
	    scrollable = scrollable.push(row);
	    // scrollable = scrollable.push(slider);
	}
	
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
