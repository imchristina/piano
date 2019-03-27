// A very odd way of simulating string motion. It sorta works.

pub struct String {
	pub length: usize, // zero indexed
	pub dispersion: f32,
	pub termination_length: usize,
	pub termination_force: f32,
	pub displacement: Vec<f32>,
	pub velocity: Vec<f32>,
}

impl String {
	pub fn update(&mut self) -> f32 {
		for i in 0..self.length { // calculate forces
			let difference = (self.displacement[i]-self.displacement[i+1])*self.dispersion;
			self.velocity[i+1] +=  difference;
			self.velocity[i] -= difference;
		}

		for i in 0..self.length+1 {
			self.displacement[i] += self.velocity[i];
		}
		
		for i in 0..self.termination_length { // soft terminations/kinda tension
			self.displacement[i] *= self.termination_force*(i+1) as f32; // left
		}
		
		let output = self.velocity[0];
		self.displacement[0] = 0_f32; // rigid terminations
		self.displacement[self.length] = 0_f32;
		self.velocity[0] = 0_f32;
		self.velocity[self.length] = 0_f32;
		
		output
	}
}
