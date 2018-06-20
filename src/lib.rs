const POINTS: usize = 200; // defaults
const DISPERSION: f32 = 1.0_f32;
const LOSS: f32 = 1.0_f32;
const TERMINATION_POINTS: usize = 2;
const SUBSAMPLING: usize = 5;

#[macro_use]

extern crate vst;
use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::api::Events;
mod string;
mod hammer;
use hammer::hammer as hammer;
mod event;

struct Piano {
	notes: Vec<event::note>,
}

impl Default for Piano {
	fn default() -> Piano {
		Piano {
			notes: Vec::new(),
		}
	}
}

impl Plugin for Piano {
	fn init(&mut self) {
		for i in 0..128 {
			self.notes.push(event::new(POINTS, (i as f32/127_f32), LOSS, TERMINATION_POINTS, SUBSAMPLING)); // 2_f32.powf(0.13*(127-i) as f32) as usize + 50
		}
	}
	fn get_info(&self) -> Info {
		Info {
			name: "Piano".to_string(),
			unique_id: 0,
			inputs: 0,
			outputs: 1,
			parameters: 5,
			category: Category::Synth,
			..Default::default()
		}
	}
	/*fn get_parameter(&self, index: i32) -> f32 {
		match index {
			0 => (self.points/1000) as f32,
			1 => self.dispersion,
			2 => self.loss,
			3 => (self.termination_points/10) as f32,
			4 => 0_f32,
			_ => 0.0,
		}
	}*/
	/*fn set_parameter(&mut self, index: i32, value: f32) {
		match index {
			0 => self.points = (value*1000_f32) as usize,
			1 => self.dispersion = value,
			2 => self.loss = value,
			3 => self.termination_points = (value*10_f32) as usize,
			4 => if value > 0_f32 {
					self.string = string::new(self.points, self.dispersion, self.loss, self.termination_points);
				},
			_ => (),
		}
	}*/
	fn get_parameter_name(&self, index: i32) -> String {
		match index {
			0 => "points".to_string(),
			1 => "dispersion".to_string(),
			2 => "loss".to_string(),
			3 => "soft termination points".to_string(),
			4 => "reload".to_string(),
			_ => "".to_string(),
		}
	}
	/*fn get_parameter_text(&self, index: i32) -> String {
		match index {
			0 => format!("{}", self.points),
			1 => format!("{}", self.dispersion),
			2 => format!("{}", self.loss),
			3 => format!("{}", self.termination_points),
			_ => "".to_string(),
		}
	}*/
	fn get_parameter_label(&self, index: i32) -> String {
		match index {
			0 => "".to_string(),
			1 => "/1".to_string(),
			2 => "/1".to_string(),
			3 => "".to_string(),
			_ => "".to_string(),
		}
	}
	fn process_events(&mut self, events: &Events) {
		for event in events.events() {
			match event {
				Event::Midi(ev) => {
					match ev.data[0] {
						128 => { // note off
							self.notes[ev.data[1] as usize].active = false;
						},
						144 => { // note on
							self.notes[ev.data[1] as usize].active = true;
							self.notes[ev.data[1] as usize].time = 0;
						},
						_ => (),
					}
				},
				_ => (),
			}
		}
	}
	fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
		let (_, output_buffer) = buffer.split();
		for output_channel in output_buffer.into_iter() {
			for output_sample in output_channel {
				let (right, left) = event::update(&mut self.notes);
				*output_sample = (left/2_f32)+(right/2_f32);
			}
		}
	}
}

plugin_main!(Piano);
