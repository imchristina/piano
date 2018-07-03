#[macro_use]

extern crate vst;
use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::api::Events;
mod string;
mod hammer;
mod event;
mod tuning;

struct Piano {
	notes: Vec<event::Note>,
	sample_rate: usize,
	length: usize,
	last_note: u8,
	sustain: bool,
}

impl Default for Piano {
	fn default() -> Piano {
		Piano {
			notes: Vec::new(),
			sample_rate: 44100,
			length: 0,
			last_note: 0,
			sustain: false,
		}
	}
}

impl Plugin for Piano {
	fn init(&mut self) {
		for i in 0..128 {
			self.notes.push(tuning::note(i, self.sample_rate));
		}
	}
	fn get_info(&self) -> Info {
		Info {
			name: "Piano".to_string(),
			unique_id: 0,
			inputs: 0,
			outputs: 1,
			parameters: 7,
			category: Category::Synth,
			..Default::default()
		}
	}
	fn get_parameter(&self, index: i32) -> f32 {
		match index {
			0 => self.last_note as f32/127_f32,
			1 => (self.length/1000) as f32,
			2 => self.notes[self.last_note as usize].string.dispersion,
			3 => self.notes[self.last_note as usize].string.loss,
			4 => (self.notes[self.last_note as usize].string.termination_points/10) as f32,
			5 => self.notes[self.last_note as usize].subsampling as f32/10_f32,
			6 => 0_f32,
			_ => 0.0,
		}
	}
	fn set_parameter(&mut self, index: i32, value: f32) {
		match index {
			1 => self.length = (value*1000_f32) as usize,
			2 => self.notes[self.last_note as usize].string.dispersion = value,
			3 => self.notes[self.last_note as usize].string.loss = value,
			4 => self.notes[self.last_note as usize].string.termination_points = (value*10_f32) as usize,
			5 => self.notes[self.last_note as usize].subsampling = (value*10_f32) as usize,
			6 => if value > 0_f32 {
					self.notes[self.last_note as usize].string.y = vec!(0_f32; self.length);
					self.notes[self.last_note as usize].string.v = vec!(0_f32; self.length);
					self.notes[self.last_note as usize].string.length = self.length;
				},
			_ => (),
		}
	}
	fn get_parameter_name(&self, index: i32) -> String {
		match index {
			0 => "note".to_string(),
			1 => "length".to_string(),
			2 => "dispersion".to_string(),
			3 => "loss".to_string(),
			4 => "soft termination points".to_string(),
			5 => "subsampling".to_string(),
			6 => "apply length change".to_string(),
			_ => "".to_string(),
		}
	}
	fn get_parameter_text(&self, index: i32) -> String {
		match index {
			0 => format!("{}", self.last_note),
			1 => format!("{}", self.length),
			2 => format!("{}", self.notes[self.last_note as usize].string.dispersion),
			3 => format!("{}", self.notes[self.last_note as usize].string.loss),
			4 => format!("{}", self.notes[self.last_note as usize].string.termination_points),
			_ => "".to_string(),
		}
	}
	fn get_parameter_label(&self, index: i32) -> String {
		match index {
			0 => "/127".to_string(),
			1 => "".to_string(),
			2 => "/1".to_string(),
			3 => "/1".to_string(),
			4 => "/10".to_string(),
			_ => "".to_string(),
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
							self.last_note = ev.data[1];
							self.length = self.notes[ev.data[1] as usize].string.length;
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
