use crate::hybrid_string::{HybridString, HybridStringConfig};
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
	pub fn note_on(&mut self, key: u8, velocity: u8, tuning: &Tuning) { 
		let tune = tune(key, tuning);
		let waveguide_length = if tune > tuning.filter_length {
			(tune - tuning.filter_length)
		} else {
			1
		};
		
		let mut adjusted_displacement: Vec<f32> = Vec::with_capacity(waveguide_length);
		let mut last_sample = 0_f32;
		for i in 0..waveguide_length {
			let mut avg = ((tuning.initial_displacement[i]-tuning.displacement_avg[waveguide_length-1]) + last_sample*tuning.pluck_damping) / (1_f32+tuning.pluck_damping);
			avg *= velocity as f32/127.0;
			adjusted_displacement.push(avg);
			last_sample = avg;
		}
		
		let string_config = HybridStringConfig {
			waveguide_length,
			differential_length: tuning.filter_length,
			dispersion: tuning.dispersion,
			soft_termination_length: tuning.filter_termination_length,
			soft_termination_force: tuning.filter_termination_force,
			initial_displacement: adjusted_displacement,
		};
		
		self.notes.push_front(Note {
			key,
			key_down: true,
			string: HybridString::new(string_config),
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
	string: HybridString,
	sub_sampling: usize,
}

pub struct Tuning {
	pub dispersion: f32,
	pub filter_length: usize,
	pub filter_termination_length: usize,
	pub filter_termination_force: f32,
	pub initial_displacement: Vec<f32>,
	pub displacement_avg: Vec<f32>,
	pub sample_rate: f32,
	pub a4_frequency: f32,
	pub sub_sampling: usize,
	pub pluck_damping: f32,
}

fn tune(key: u8, tuning: &Tuning) -> usize {
	((tuning.a4_frequency/(48000.0/tuning.sample_rate)*tuning.sub_sampling as f32)*2_f32.powf(1_f32/12_f32).powf(-(key as f32-45_f32))) as usize
}
