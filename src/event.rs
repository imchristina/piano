use crate::karplus_strong::KarplusStrong;
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
		let length = tune(key, tuning);
		let mut delay: VecDeque<f32> = VecDeque::with_capacity(length);
		for i in 0..length {
			delay.push_front(tuning.initial_displacement[i]);
		}
		self.notes.push_front(Note {
			key,
			key_down: true,
			string: KarplusStrong {
				average_weight: 0.25_f32,
				delay,
				previous_sample: 0_f32,
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
	string: KarplusStrong,
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

fn tune(key: u8, tuning: &Tuning) -> usize {
	((tuning.a4_frequency/(48000.0/tuning.sample_rate)*tuning.sub_sampling as f32)*2_f32.powf(1_f32/12_f32).powf(-(key as f32-45_f32))) as usize
}
