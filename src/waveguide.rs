use std::collections::VecDeque;

pub struct String {
	pub length: usize,
	pub r: Vec<f32>,
	pub l: Vec<f32>,
	start: usize,
	end: usize,
	// Filters
	disperse_r: ThiranAllPassFilter,
	disperse_l: ThiranAllPassFilter,
}

impl String {
	pub fn update(&mut self) -> (f32, f32) {
		let mut end_r = self.r[self.end];
		let mut end_l = self.l[self.end];
		
		end_r = self.disperse_r.update(end_r);  // https://ccrma.stanford.edu/~jos/pasp/Dispersive_Traveling_Waves.html
		end_l = self.disperse_l.update(end_l);
		
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

pub fn new(length: usize, dispersion_delay: f32) -> String {
	String {
		length: length - 1,
		r: vec![0_f32; length],
		l: vec![0_f32; length],
		start: 0,
		end: 1,
		disperse_r: ThiranAllPassFilter::new(dispersion_delay, 2),
		disperse_l: ThiranAllPassFilter::new(dispersion_delay, 2),
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
	input: VecDeque<f32>,
	output: VecDeque<f32>,
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
		let buffer_size = (n*2)+1;
		let mut input: VecDeque<f32> = VecDeque::with_capacity(buffer_size);
		let mut output: VecDeque<f32> = VecDeque::with_capacity(buffer_size);
		for _i in 0..buffer_size {
			input.push_back(0_f32);
			output.push_back(0_f32);
		}
		Self {
			a,
			b,
			n,
			input,
			output,
		}
	}
	
	fn update(&mut self, input: f32) -> f32 {
		self.input.pop_front();
		self.input.push_back(input);
		self.output.pop_front();
		self.output.push_back(input);  // not sure if this is how the output is supposed to be handled
		
		for n in 0..self.n+1 {
			for i in 0..self.n+1 {
				let input_mul = *self.input.get(n-i+self.n).unwrap();
				let output = *self.output.get_mut(n-i+self.n).unwrap();
				let output_add = self.a[i]*input_mul+self.b[i]*output; // this whole thing is super ugly
				let output = self.output.get_mut(n).unwrap();
				*output += output_add;
			}
		}
		*self.output.back().unwrap()
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
