use filter::ThiranAllPass;

pub struct Waveguide {
	pub length: usize,
	pub r: Vec<f32>,
	pub l: Vec<f32>,
	start: usize,
	end: usize,
	// Filters
	disperse_r: ThiranAllPass,
	disperse_l: ThiranAllPass,
}

impl Waveguide {
	pub fn new(length: usize, dispersion_delay: f32) -> Self {
		Self {
			length: length - 1,
			r: vec![0_f32; length],
			l: vec![0_f32; length],
			start: 0,
			end: 1,
			disperse_r: ThiranAllPass::new(dispersion_delay, 2),
			disperse_l: ThiranAllPass::new(dispersion_delay, 2),
		}
	}
	pub fn update(&mut self) -> (f32, f32) {
		let mut end_r = self.r[self.end];
		let mut end_l = self.l[self.end];
		
		end_r = self.disperse_r.update(0_f32);  // https://ccrma.stanford.edu/~jos/pasp/Dispersive_Traveling_Waves.html
		end_l = self.disperse_l.update(0_f32);
		
		self.r[self.start] = -end_l;
		self.l[self.start] = -end_r;
		
		self.start += 1;
		self.end += 1;
		if self.start > self.length {
			self.start = 0;
		} else if self.end > self.length {
			self.end = 0;
		}
		
		(end_r, end_l)
	}
	
	pub fn get_displacement(&self, position: usize) -> f32 {
		self.r[position]+self.l[self.length-position]
	}
	
	pub fn set_displacement(&mut self, position: usize, mut displacement: f32) {
		displacement /= 2_f32;
		self.r[position] = displacement;
		self.l[self.length-position] = displacement;
	}
}
