use event;

pub fn note(note: u8, _sample_rate: usize) -> event::Note {
	let subsampling = 4;
	let mut points = (440_f32*(2_f32.powf(1_f32/12_f32)).powf(-(note as f32-67_f32))) as usize;
	
	if points < 30 {
		points = 30;
	}
	
	event::new(points, subsampling, 6.157_f32, 2)  //6.15782
}
