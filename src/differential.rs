pub struct Differential {
	pub length: usize, // number of points in string
	pub dispersion: f32, // coefficient of energy transfered into surrounding points, does not introduce loss
	pub loss: f32,
	pub termination_points: usize,
	pub termination_force: f32,
	pub y: Vec<f32>,
	pub v: Vec<f32>
}

impl Differential {
	pub fn new(length: usize, dispersion: f32, loss: f32, termination_points: usize) -> Self {
		let y: Vec<f32> = vec![0_f32; length];
		let v: Vec<f32> = vec![0_f32; length];
		let termination_force = 0.5_f32/termination_points as f32;
		Self {
			length: length-1, // make length zero-indexed
			dispersion,
			loss,
			termination_points,
			termination_force,
			y,
			v
		}
	}
	pub fn update(&mut self) -> (f32, f32) {
		for i in 1..self.length+1 { // calculate forces
			let energy = (self.y[i]-self.y[i-1])*self.dispersion;
			self.v[i-1] = (self.v[i-1] + energy) * self.loss;
			self.v[i] -= energy;
			self.y[i-1] += self.v[i-1]; // might be better to do loss here
		}
		for i in 0..self.termination_points { // soft terminations
			self.y[i] *= self.termination_force*(i as f32); // left
			self.y[(self.length-self.termination_points)+i] *= 1_f32-(self.termination_force*i as f32); // right
		}
		let force_left = self.v[0];
		let force_right = self.v[self.length];
		self.y[0] = 0_f32; // rigid terminations
		self.y[self.length] = 0_f32;
		self.v[0] = 0_f32;
		self.v[self.length] = 0_f32;
		(force_left, force_right) // return v forces at termination points
	}
}
