pub struct string {
	dispersion: f32,
	loss: f32,
	termination_points: usize,
	y: Vec<f32>,
	v: Vec<f32>
}

pub fn new(size: usize, dispersion: f32, loss: f32, termination_points: usize) -> string {
	let y: Vec<f32> = vec![0_f32; size];
	let v: Vec<f32> = vec![0_f32; size];
	string {
		dispersion,
		loss,
		termination_points,
		y,
		v
	}
}

pub fn update(s: &mut string) {
	
}
