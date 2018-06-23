use string;
use hammer::hammer as hammer;

pub struct Note {
	// String
	pub string: string::String, // TODO more than one string per note
	// Hammer
	pub time: f32,
	pub velocity: f32,
	// General
	pub subsampling: usize,
	pub active: bool,
}

pub fn new(points: usize, dispersion: f32, loss: f32, termination_points: usize, velocity: f32, subsampling: usize) -> Note {
	Note {
		string: string::new(points, dispersion, loss, termination_points),
		time: 0_f32,
		velocity,
		subsampling,
		active: false,
	}
}

pub fn update(notes: &mut Vec<Note>, dt: f32) -> (f32, f32) { // Audio signal path, does hammer, string, and soundboard calculations
	let mut out_left = 0_f32;
	let mut out_right = 0_f32;
	let mut active_notes = 0; // could be useful later
	for note in notes {
		if note.active {
			let mut left = 0_f32;
			let mut right = 0_f32;
			for _i in 0..note.subsampling {
				hammer(&mut note.string, 0.9_f32, 5, note.velocity, 10, note.time, dt); // TODO remove hardcoded values
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
