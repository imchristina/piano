use event;

pub fn note(note: u8, velocity: f32, sample_rate: usize) -> event::Note { //points: usize, dispersion: f32, loss: f32, termination_points: usize, velocity: f32, subsampling: usize
	let points = note_to_points(note, sample_rate); //(0.09*note as f32).exp() as usize+50;
	let dispersion = 0.2_f32;
	let loss = 1_f32;
	let termination_points = 2;
	let subsampling = 2;
	
	match note {
		_ => ()
	}
	
	event::new(points, dispersion, loss, termination_points, velocity, subsampling)
}

fn note_to_points(note: u8, sample_rate: usize) -> usize {
	let note_up_scalar = (2_f32).powf(1_f32/12_f32);
	let midi_0 = 8.1758;
	
	//(2_f32/(2_f32*(1_f32/sample_rate as f32)*(midi_0*(note_up_scalar*(note+1) as f32)))) as usize
	((1_f32-(note as f32).log(127_f32))*500_f32) as usize+50
}
