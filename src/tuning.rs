use event;

pub fn note(note: u8, sample_rate: usize) -> event::Note { //points: usize, dispersion: f32, loss: f32, termination_points: usize, velocity: f32, subsampling: usize
	let dispersion = 0.2_f32;
	let loss = 1_f32;
	let termination_points = 2;
	let subsampling = 2;
	let points = (440_f32*(1.059463_f32).powf((19_f32*(subsampling-1) as f32)-(note as f32-11_f32))) as usize;
	
	match note {
		_ => ()
	}
	
	event::new(points, dispersion, loss, termination_points, subsampling)
}
