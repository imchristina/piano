use event::Note;

pub fn note(note: u8, _sample_rate: usize) -> Note {
	let subsampling = 1;
	let mut points = (440_f32*(2_f32.powf(1_f32/12_f32)).powf(-(note as f32-32_f32))) as usize; //67
	
	if points < 30 {
		points = 30;
	}
	
	Note::new(points, subsampling, 6.157_f32, 2)  //6.15782
}
