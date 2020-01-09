#[macro_use]

extern crate vst;
use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::api::Events;
mod hybrid_string;
extern crate rand;
use self::rand::Rng;
mod event;
use event::{EventManager, Tuning};

struct Piano {
	event_manager: EventManager,
	tuning: Tuning,
	midi_queue: Vec<vst::event::MidiEvent>,
	displacement_type: usize,
}

impl Default for Piano {
	fn default() -> Piano {
		Piano {
			event_manager: EventManager::new(),
			tuning: Tuning {
				dispersion: 1_f32,
				filter_length: 10,
				filter_termination_length: 3,
				filter_termination_force: 0.33,
				initial_displacement: Vec::new(),
				displacement_avg: Vec::new(),
				sample_rate: 48000.0,
				a4_frequency: 440.0,
				sub_sampling: 1,
				pluck_damping: 1.0,
			},
			midi_queue: Vec::new(),
			displacement_type: 0,
		}
	}
}

impl Plugin for Piano {
	fn get_info(&self) -> Info {
		Info {
			name: "String Plugin".to_string(),
			unique_id: 0,
			inputs: 0,
			outputs: 1,
			parameters: 3,
			category: Category::Synth,
			..Default::default()
		}
	}
	fn init(&mut self) {
		let mut rng = rand::thread_rng();
		let mut sum = 0_f32;
		match self.displacement_type {
			0 => {
				for i in 1..25000 {
					let v = (rng.gen::<f32>()-0.5)*2.0;
					self.tuning.initial_displacement.push(v);
					sum += v;
					self.tuning.displacement_avg.push(sum/i as f32);
				}
			} 1 => {
				for i in 1..25000 {
					if i < 50 {
						let v = (rng.gen::<f32>()-0.5)*2.0;
						self.tuning.initial_displacement.push(v);
						sum += v;
						self.tuning.displacement_avg.push(sum/i as f32);
					} else {
						self.tuning.initial_displacement.push(0.0);
						self.tuning.displacement_avg.push(sum/i as f32);
					}
				}
			} _ => (),
		}
	}
	fn process_events(&mut self, events: &Events) {
		for event in events.events() {
			match event {
				Event::Midi(ev) => {
					self.midi_queue.push(ev);
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
			for ev in &mut self.midi_queue {
				if ev.delta_frames as usize == i {
					match ev.data[0] {
						128 => { // note off
							self.event_manager.note_off(ev.data[1]);
						},
						144 => { // note on
							self.event_manager.note_on(ev.data[1], ev.data[2], &self.tuning);
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
				}
			}
			
			let sample = self.event_manager.strings_update();

			// Write the same sample to each of the channels (make it mono)
			for out in output_buffer {
				out[i] = sample;
			}
		}
		self.midi_queue.clear()
	}
	fn get_parameter_name(&self, index: i32) -> String {
		match index {
			0 => "Sub Sampling".to_string(),
			1 => "Dispersion".to_string(),
			2 => "Pluck Damping".to_string(),
			_ => "".to_string()
		}
	}
	fn get_parameter(&self, index: i32) -> f32 {
		match index {
			0 => self.tuning.sub_sampling as f32/10_f32,
			1 => self.tuning.dispersion,
			2 => self.tuning.pluck_damping,
			_ => 0.0
		}
	}
	fn get_parameter_text(&self, index: i32) -> String {
		match index {
			0 => self.tuning.sub_sampling.to_string(),
			_ => "".to_string()
		}
	}
	fn get_parameter_label(&self, index: i32) -> String {
		match index {
			0 => "X".to_string(),
			_ => "".to_string()
		}
	}
	fn set_parameter(&mut self, index: i32, value: f32) {
		match index {
			0 => self.tuning.sub_sampling = (1.0+(value*5_f32)) as usize,
			1 => self.tuning.dispersion = value,
			2 => self.tuning.pluck_damping = value,
			_ => ()
		}
	}
}

plugin_main!(Piano);
