pub struct String {
	pub length: usize,
	pub r: Vec<f32>,
	pub l: Vec<f32>,
	start: usize,
	end: usize,
	// Filters
	disperse_r: ThiranAllPassFilter,
	disperse_l: ThiranAllPassFilter,
	end_r_buffer: Vec<f32>,
	end_l_buffer: Vec<f32>,
}

impl String {
	pub fn update(&mut self) -> (f32, f32) {
		self.end_r_buffer.push(self.r[self.end]);
		self.end_l_buffer.push(self.l[self.end]);
		self.end_r_buffer.pop();
		self.end_l_buffer.pop();
		
		self.end_r_buffer = self.disperse_r.update(&self.end_r_buffer);  // https://ccrma.stanford.edu/~jos/pasp/Dispersive_Traveling_Waves.html
		self.end_l_buffer = self.disperse_l.update(&self.end_l_buffer);
		let end_r = self.end_r_buffer[1];
		let end_l = self.end_l_buffer[1];
		
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

pub fn new(length: usize) -> String {
	String {
		length: length - 1,
		r: vec![0_f32; length],
		l: vec![0_f32; length],
		start: 0,
		end: 1,
		disperse_r: ThiranAllPassFilter::new(2_f32, 2), //6.15782
		disperse_l: ThiranAllPassFilter::new(2_f32, 2),
		end_r_buffer: vec![0_f32; 3],
		end_l_buffer: vec![0_f32; 3],
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

struct ThiranAllPassFilter { // https://ccrma.stanford.edu/~jos/pasp/Thiran_Allpass_Interpolators.html
	a: Vec<f32>,
	b: Vec<f32>,
	n: usize,
}

impl ThiranAllPassFilter {
	fn new(delay: f32, n: usize) -> Self {
		let mut a = vec![0_f32; n+1];
		let mut b = vec![0_f32; n+1];
		for k in 0..n+1 {
			let mut ak = 1_f32;
			for i in 0..n+1 {
				ak *= (delay-(n+i) as f32)/(delay-(n+k+i) as f32);
			}
			let out = (-1_f32).powi(k as i32) * n_choose_k(n, k) as f32 * ak;
			a[k] = out;
			b[n-k] = out;
		}
		Self {
			a,
			b,
			n,
		}
	}
	
	fn update(&mut self, input: &Vec<f32>) -> Vec<f32> {
		let mut output = vec![0_f32; self.n+1];
		for out_n in 0..self.n+1 {
			for n in 0..self.n+1 {
				output[out_n] += self.a[n]*input[n]+self.b[n]*output[n];
			}
		}
		output
	}
}

fn n_choose_k(n: usize, k: usize) -> usize { // https://math.stackexchange.com/a/927064
	if k == 0 {
		1
	} else if k > n/2 {
		n_choose_k(n, n-k)
	} else {
		n * n_choose_k(n-1,k-1) / k
	}
}
