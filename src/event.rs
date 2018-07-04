use string;
use hammer::hammer as hammer;
use hammer::damper as damper;

pub struct Note {
	// String
	pub string: string::String, // TODO more than one string per note
	// Hammer
	pub time: f32,
	pub velocity: f32,
	pub damper: bool,
	// General
	pub subsampling: usize,
	pub active: bool,
	pub released: bool,
}

pub fn new(points: usize, dispersion: f32, loss: f32, termination_points: usize, subsampling: usize) -> Note {
	Note {
		string: string::new(points, dispersion, loss, termination_points),
		time: 0_f32,
		velocity: 0_f32,
		damper: false,
		subsampling,
		active: false,
		released: false,
	}
}

pub fn update(notes: &mut Vec<Note>, dt: f32, sustain: bool) -> (f32, f32) { // Audio signal path, does hammer, string, and soundboard calculations
	let mut out_left = 0_f32;
	let mut out_right = 0_f32;
	let mut active_notes = 0; // could be useful later
	for note in notes {
		if note.active {
			if note.released {
				if !sustain {
					note.damper = true;
					note.released = false;
					note.time = 0_f32;
				}
			}
			
			let mut left = 0_f32;
			let mut right = 0_f32;
			for _i in 0..note.subsampling {
				if note.damper {
					if damper(&mut note.string, 0.5_f32, 10, 10, note.time) {
						note.damper = false;
						note.active = false;
					}
				} else {
					hammer(&mut note.string, 0.9_f32, 5, note.velocity, 10, note.time); // TODO remove hardcoded values
				}
				let (string_left, string_right) = string::update(&mut note.string);
				left += string_left;
				right += string_right;
			}
			out_left += left/note.subsampling as f32;
			out_right += right/note.subsampling as f32;
			note.time += dt;
			active_notes += 1;
		}
	}
	(out_left, out_right)
}