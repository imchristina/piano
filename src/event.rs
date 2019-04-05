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
		let length = ((tuning.a4_frequency/(48000.0/tuning.sample_rate))*2_f32.powf(1_f32/12_f32).powf(-(key as f32-45_f32))) as usize;
		self.notes.push_front(Note {
			key,
			string: String {
				length,
				dispersion: tuning.dispersion,
				termination_length: tuning.termination_length,
				termination_force: tuning.termination_force,
				displacement: vec![0.0; length+1],
				velocity: tuning.initial_displacement.clone(), // TODO fixed inital displacement size
			},
			key_down: true,
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
			output += note.string.update()
		}
		output
	}
}

pub struct Note {
	key: u8,
	string: String,
	key_down: bool,
}

pub struct Tuning {
	pub dispersion: f32,
	pub termination_length: usize,
	pub termination_force: f32,
	pub initial_displacement: Vec<f32>,
	pub sample_rate: f32,
	pub a4_frequency: f32,
}
