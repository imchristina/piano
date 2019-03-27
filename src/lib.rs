#[macro_use]

extern crate vst;
use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::api::Events;
extern crate rand;
use self::rand::Rng;
mod string;
use string::String;

struct Piano {
	note: String,
	init_displacement: Vec<f32>,
}

impl Default for Piano {
	fn default() -> Piano {
		Piano {
			note: String {
				length: 500-1,
				dispersion: 1.0,
				termination_length: 2,
				termination_force: 1.0/3.0,
				displacement: vec![0.0; 500],
				velocity: vec![0.0; 500],
			}, 
			init_displacement: Vec::new(),
		}
	}
}

impl Plugin for Piano {
	fn get_info(&self) -> Info {
		Info {
			name: "String Test".to_string(),
			unique_id: 0,
			inputs: 0,
			outputs: 1,
			parameters: 0,
			category: Category::Synth,
			..Default::default()
		}
	}
	fn init(&mut self) {
		let mut rng = rand::thread_rng();
		for _i in 0..500 {
			self.init_displacement.push((rng.gen::<f32>()-0.5)*2.0)
		}
	}
	fn process_events(&mut self, events: &Events) {
		for event in events.events() {
			match event {
				Event::Midi(ev) => {
					match ev.data[0] {
						128 => { // note off
						},
						144 => { // note on
							self.note.displacement = vec![0.0;500];
							self.note.velocity = self.init_displacement.clone();
						},
						176 => { // control (pedals)
							match ev.data[1] {
								64 => { // sustain/damper pedal
									if ev.data[2] >= 64 { // pedal on
										
									} else { // pedal off
										
									}
								},
								_ => (),
							}
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
				*output_sample = self.note.update();
			}
		}
	}
}

plugin_main!(Piano);
