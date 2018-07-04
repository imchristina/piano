pub struct String {
	pub length: usize,
	pub r: Vec<f32>,
	pub l: Vec<f32>,
	pointer: usize, // end of string, +1 is start of string
	// Filters
	disperse_r: AllPassFilter,
	disperse_l: AllPassFilter,
}

impl String {
	pub fn update(&mut self) -> (f32, f32) { // TODO put this in the impl
		let mut end_r = self.r[self.pointer];
		let mut end_l = self.l[self.pointer];
		
		end_r = self.disperse_r.update(end_r);  // https://ccrma.stanford.edu/~jos/pasp/Dispersive_Traveling_Waves.html
		end_l = self.disperse_l.update(end_l);
		
		self.r[self.pointer+1] = -end_l;
		self.l[self.pointer+1] = -end_r;
		
		self.pointer += 1;
		if self.pointer >= self.length {
			self.pointer = 0;
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

pub fn new(length: usize) -> String {
	String {
		length: length - 1,
		r: vec![0_f32; length],
		l: vec![0_f32; length],
		pointer: 0,
		disperse_r: AllPassFilter::new(0.5_f32, 2),
		disperse_l: AllPassFilter::new(0.5_f32, 2),
	}
}

struct AllPassFilter {
	gain: f32,
	delay: Vec<f32>,
	pointer: usize,
}

impl AllPassFilter {
	fn new(gain: f32, size: usize) -> Self {
		Self {
			gain,
			delay: vec![0_f32; size],
			pointer: 0,
		}
	}
	
	fn update(&mut self, input: f32) -> f32 {
		let output = self.delay[self.pointer]+(input*-self.gain);
		self.delay[self.pointer+1] = input+(output*self.gain);
		
		self.pointer += 1;
		if self.pointer >= self.delay.len()-1 {
			self.pointer = 0;
		}
		output
	}
}
