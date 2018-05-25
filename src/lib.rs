const POINTS: usize = 200;

#[macro_use]
extern crate vst;
use vst::plugin::{Info, Plugin};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::api::Events;
mod string;

struct Piano {
	string: string::String,
}

impl Default for Piano {
	fn default() -> Piano {
		Piano {
			string: string::new(POINTS, 1_f32, 1_f32, 3)
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
			..Default::default()
		}
	}
	fn process_events(&mut self, events: &Events) {
		for event in events.events() {
			match event {
				Event::Midi(ev) => {
					//midi stuff
					self.string = string::new(POINTS, 1_f32, 1_f32, 3);
				}, _ => ()
			}
		}
	}
	fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
		let (_, output_buffer) = buffer.split();
		for output_channel in output_buffer.into_iter() {
			for output_sample in output_channel {
				let (left, right) = string::update(&mut self.string);
				*output_sample = left/10_f32
			}
		}
	}
}

plugin_main!(Piano);

#[cfg(test)]
mod tests {
	use string;
    #[test]
    fn test_string_setup() {
        let mut string = string::new(100, 1_f32, 1_f32, 3);
    }
	#[test]
	fn test_string_update() {
		let mut string = string::new(100, 1_f32, 1_f32, 3);
		string::update(&mut string);
	}
}
