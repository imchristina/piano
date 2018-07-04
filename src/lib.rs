#[macro_use]

extern crate vst;
use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::api::Events;
mod waveguide;
use waveguide as string;
mod hammer;
mod event;
mod tuning;

struct Piano {
	notes: Vec<event::Note>,
	sample_rate: usize,
	sustain: bool,
}

impl Default for Piano {
	fn default() -> Piano {
		Piano {
			notes: Vec::new(),
			sample_rate: 44100,
			sustain: false,
		}
	}
}

impl Plugin for Piano {
	fn get_info(&self) -> Info {
		Info {
			name: "Piano".to_string(),
			unique_id: 0,
			inputs: 0,
			outputs: 1,
			parameters: 0,
			category: Category::Synth,
			..Default::default()
		}
	}
	fn init(&mut self) {
		for i in 0..128 {
			self.notes.push(tuning::note(i, self.sample_rate));
		}
	}
	fn process_events(&mut self, events: &Events) {
		for event in events.events() {
			match event {
				Event::Midi(ev) => {
					match ev.data[0] {
						128 => { // note off
							self.notes[ev.data[1] as usize].released = true;
						},
						144 => { // note on
							self.notes[ev.data[1] as usize].active = true;
							self.notes[ev.data[1] as usize].damper = false;
							self.notes[ev.data[1] as usize].time = 0_f32;
							self.notes[ev.data[1] as usize].velocity = ev.data[2] as f32 / 12_f32;
						},
						176 => { // control (pedals)
							match ev.data[1] {
								64 => { // sustain/damper pedal
									if ev.data[2] >= 64 { // pedal on
										self.sustain = true
									} else { // pedal off
										self.sustain = false
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
				let (right, left) = event::update(&mut self.notes, 1_f32/self.sample_rate as f32, self.sustain);
				*output_sample = (left/4_f32)+(right/4_f32);
			}
		}
	}
}

plugin_main!(Piano);
