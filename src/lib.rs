#[macro_use]

extern crate vst;
use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::api::Events;
mod string;
extern crate rand;
use self::rand::Rng;
mod event;
use event::{EventManager, Tuning};

struct Piano {
	event_manager: EventManager,
	tuning: Tuning,
}

impl Default for Piano {
	fn default() -> Piano {
		Piano {
			event_manager: EventManager::new(),
			tuning: Tuning {
				dispersion: 1.0,
				termination_length: 2,
				termination_force: 1.0/3.0,
				initial_displacement: Vec::new(),
				sample_rate: 48000.0,
				a4_frequency: 440.0,
			},
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
		for _i in 0..10000 { // TODO fixed inital displacement size
			self.tuning.initial_displacement.push((rng.gen::<f32>()-0.5)*2.0)
		}
	}
	fn process_events(&mut self, events: &Events) {
		for event in events.events() {
			match event {
				Event::Midi(ev) => {
					match ev.data[0] {
						128 => { // note off
							self.event_manager.note_off(ev.data[1]);
						},
						144 => { // note on
							self.event_manager.note_on(ev.data[1], &self.tuning);
						},
						176 => { // control (pedals)
							match ev.data[1] {
								64 => { // sustain/damper pedal
									if ev.data[2] >= 64 { // pedal on
										self.event_manager.sustain = true;
									} else { // pedal off
										self.event_manager.sustain = false;
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
	fn set_sample_rate(&mut self, rate: f32) {
		self.tuning.sample_rate = rate;
	}
	fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
		//let output_channels = buffer.output_count();
		let num_samples = buffer.samples();
		let (_, output_buffer) = buffer.split();

		for i in 0..num_samples {
			let sample = self.event_manager.update();
			// Throw away a sample
			let _ = self.event_manager.update();

			// Write the same sample to each of the channels (make it mono)
			for out in output_buffer {
				out[i] = sample;
			}

		}
	}
}

plugin_main!(Piano);
