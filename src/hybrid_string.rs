use std::collections::VecDeque;

pub struct HybridStringConfig {
	pub waveguide_length: usize,
	pub differential_length: usize,
	pub dispersion: f32,
	pub soft_termination_length: usize,
	pub soft_termination_force: f32,
	pub initial_displacement: Vec<f32>,
}

pub struct HybridString {
	waveguide: VecDeque<f32>,
	displacement: Vec<f32>,
	velocity: Vec<f32>,
	dispersion: f32,
	soft_termination_length: usize,
	soft_termination_force: f32,
	differential_length: usize,
}

impl HybridString {
	pub fn new(config: HybridStringConfig) -> Self {
		let mut initial_displacement = config.initial_displacement;
		initial_displacement.resize(config.waveguide_length, 0.0);
		
		Self {
			waveguide: VecDeque::from(initial_displacement),
			displacement: vec![0.0; config.differential_length],
			velocity: vec![0.0; config.differential_length],
			dispersion: config.dispersion,
			soft_termination_length: config.soft_termination_length,
			soft_termination_force: config.soft_termination_force,
			differential_length: config.differential_length-1,
		}
	}
	pub fn update(&mut self) -> f32 {
		self.displacement[self.differential_length-1] += -self.waveguide[0];
		self.velocity[self.differential_length-1] += -self.waveguide[0];
		self.velocity[self.differential_length] -= -self.waveguide[0];
		
		for i in 0..self.differential_length {
			let difference = (self.displacement[i]-self.displacement[i+1])*self.dispersion;
			self.velocity[i+1] += difference;
			self.velocity[i] -= difference;
		}
		
		for i in 0..self.differential_length+1 {
			self.displacement[i] += self.velocity[i];
		}
		
		for i in 1..self.soft_termination_length {
			self.displacement[i] *= self.soft_termination_force*(i+1) as f32;
		}
		
		self.displacement[0] = 0.0;
		self.velocity[0] = 0.0;
		
		let sample = self.displacement[self.differential_length];
		self.displacement[self.differential_length] = 0.0;
		self.velocity[self.differential_length] -= sample;
		self.velocity[self.differential_length-1] += sample;
		
		self.waveguide.pop_front();
		self.waveguide.push_back(sample);
		
		sample
	}
}
