const POINTS: usize = 500; //temporary
const DISPERSION: f32 = 1.0_f32;
const LOSS: f32 = 1.0_f32;
const TERMINATION_POINTS: usize = 5;

#[macro_use]

extern crate vst;
use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::api::Events;
mod string;

struct Piano {
	string: string::String,
	//strings: Vec<string::String>,
}

impl Default for Piano {
	fn default() -> Piano {
		Piano {
			string: string::new(POINTS, DISPERSION, LOSS, TERMINATION_POINTS)
		}
	}
}

impl Plugin for Piano {
	fn init(&mut self) {
		//let mut self.string = string::new(500, 1_f32, 1_f32, 3);
	}
	fn get_info(&self) -> Info {
		Info {
			name: "Piano".to_string(),
			unique_id: 0,
			inputs: 0,
			outputs: 1,
			category: Category::Synth,
			..Default::default()
		}
	}
	fn process_events(&mut self, events: &Events) {
		for event in events.events() {
			match event {
				Event::Midi(ev) => {
					match ev.data[0] {
						128 => (), // note off
						144 => self.string = string::new(POINTS-(ev.data[1]) as usize, DISPERSION, LOSS, TERMINATION_POINTS), // note on
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
				let (left, right) = string::update(&mut self.string);
				*output_sample = left
			}
		}
	}
}

plugin_main!(Piano);
