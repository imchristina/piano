pub struct String {
	length: usize,
	dispersion: f32,
	loss: f32,
	termination_points: usize,
	termination_force: f32,
	y: Vec<f32>,
	v: Vec<f32>
}

pub fn new(length: usize, dispersion: f32, loss: f32, termination_points: usize) -> String {
	let y: Vec<f32> = vec![0_f32; length];
	let mut v: Vec<f32> = vec![0_f32; length];
	v[10] = -10_f32;
	let termination_force = 1_f32/(termination_points as f32);
	String {
		length: length-1,
		dispersion,
		loss,
		termination_points,
		termination_force,
		y,
		v
	}
}

pub fn update(s: &mut String) -> (f32, f32) {
	for i in 0..s.length-1 { // calculate forces
		let energy = (s.y[i]-s.y[i+1])*s.dispersion;
		s.v[i+1] = (s.v[i+1] + energy) * s.loss;
		s.v[i] = s.v[i] - energy;
	}
	for i in 0..s.length { // apply forces
		s.y[i] = s.y[i] + s.v[i]; // might be better to do loss here
	}
	for i in 0..s.termination_points { // soft terminations
		s.y[(s.length+i-i*2)] = s.y[(s.length+i-i*2)]*(s.termination_force*i as f32);
		s.y[i] = s.y[i]*(s.termination_force*(i as f32));
	}
	let force_left = s.v[0];
	let force_right = s.v[s.length];
	s.y[0] = 0_f32;
	s.y[s.length] = 0_f32;
	s.v[0] = 0_f32;
	s.v[s.length] = 0_f32;
	(force_left, force_right) // return v forces at termination points
}
