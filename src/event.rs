use string;
use hammer::hammer as hammer;

pub struct note {
	// String
	pub string: string::String, // TODO more then one string per note
	pub points: usize,
	pub dispersion: f32,
	pub loss: f32,
	pub termination_points: usize,
	// Hammer
	pub time: usize,
	// General
	pub subsampling: usize,
	pub active: bool,
}

pub fn new(points: usize, dispersion: f32, loss: f32, termination_points: usize, subsampling: usize) -> note {
	note {
		string: string::new(points, dispersion, loss, termination_points),
		points,
		dispersion,
		loss,
		termination_points,
		time: 0,
		subsampling,
		active: false,
	}
}

pub fn update(notes: &mut Vec<note>) -> (f32, f32) { // Audio signal path, does hammer, string, and soundboard calculations
	let mut out_left = 0_f32;
	let mut out_right = 0_f32;
	let mut active_notes = 0;
	for note in notes {
		if note.active {
			let mut left = 0_f32;
			let mut right = 0_f32;
			for _i in 0..note.subsampling {
				hammer(&mut note.string, 1_f32, 5, 2_f32, 10, note.time); // TODO remove hardcoded values
				let (string_left, string_right) = string::update(&mut note.string);
				left += string_left;
				right += string_right;
			}
			out_left += left/note.subsampling as f32;
			out_right += right/note.subsampling as f32;
			note.time += 1;
			active_notes += 1;
		}
	}
	(out_left, out_right)
}
