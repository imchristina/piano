use crate::string::String;
use std::collections::VecDeque;

pub struct EventManager {
	notes: VecDeque<Note>,
	pub sustain: bool,
}

impl EventManager {
	pub fn new() -> Self {
		Self {
			notes: VecDeque::new(),
			sustain: false,
		}
	}
	pub fn note_on(&mut self, key: u8, tuning: &Tuning) {
		let length = tune(key, tuning.a4_frequency, tuning.sample_rate);
		self.notes.push_front(Note {
			key,
			key_down: true,
			string: String {
				length,
				dispersion: tuning.dispersion,
				termination_length: tuning.termination_length,
				termination_force: ((1.0-length as f32/tune(0, tuning.a4_frequency, tuning.sample_rate) as f32)/2.0)*tuning.termination_force,
				displacement: vec![0.0; length+1],
				velocity: tuning.initial_displacement.clone(), // TODO fixed inital displacement size
			},
			sub_sampling: tuning.sub_sampling,
		})
	}
	pub fn note_off(&mut self, key: u8) {
		for note in &mut self.notes {
			if note.key == key {
				note.key_down = false;
			}
		}
	}
	pub fn strings_update(&mut self) -> f32 {
		if !self.sustain {
			self.notes.retain(|note| note.key_down);
		}
		let mut output = 0_f32;
		for note in &mut self.notes {
			output += note.string.update();
			for _i in 1..note.sub_sampling {
				let _ = note.string.update();
			}
		}
		output
	}
}

pub struct Note {
	key: u8,
	key_down: bool,
	string: String,
	sub_sampling: usize,
}

pub struct Tuning {
	pub dispersion: f32,
	pub termination_length: usize,
	pub termination_force: f32,
	pub initial_displacement: Vec<f32>,
	pub sample_rate: f32,
	pub a4_frequency: f32,
	pub sub_sampling: usize,
}

fn tune(key: u8, a4: f32, sample_rate: f32) -> usize {
	((a4/(48000.0/sample_rate))*2_f32.powf(1_f32/12_f32).powf(-(key as f32-45_f32))) as usize
}
