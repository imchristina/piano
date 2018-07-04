use event;

pub fn note(note: u8, _sample_rate: usize) -> event::Note {
	let subsampling = 1;
	let points = (440_f32*(1.059463_f32).powf(-(note as f32-44_f32))) as usize;
	
	event::new(points, subsampling)
}

/*pub fn _note_differential(note: u8, _sample_rate: usize) -> event::Note {
	let mut dispersion = 1_f32;
	let loss = 1_f32;
	let termination_points = 2;
	let mut subsampling = 2;
	let mut points = (440_f32*(1.059463_f32).powf(-(note as f32-44_f32))) as usize;//(440_f32*(1.059463_f32).powf((19_f32*1_f32)-(note as f32-69_f32))) as usize;
	
	if points < 50 {
		dispersion *= 50_f32/points as f32;
		if dispersion > 1_f32 {
			dispersion = 1_f32;
		}
		subsampling *= (50_f32/points as f32) as usize;
		points = 50;
	}
	
	event::new(points, dispersion, loss, termination_points, subsampling)
}*/
