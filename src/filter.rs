use std::collections::VecDeque;

pub struct IntegerAllPass { // Generic integer-delay allpass
	gain: f32,
	delay: Vec<f32>,
	pointer: usize,
}

impl IntegerAllPass {
	pub fn new(gain: f32, size: usize) -> Self {
		Self {
			gain,
			delay: vec![0_f32; size],
			pointer: 0,
		}
	}
	
	pub fn update(&mut self, input: f32) -> f32 {
		let output = self.delay[self.pointer]+(input*-self.gain);
		self.delay[self.pointer+1] = input+(output*self.gain);
		
		self.pointer += 1;
		if self.pointer >= self.delay.len()-1 {
			self.pointer = 0;
		}
		output
	}
}

pub struct Filter {
	a: Vec<f32>,
	b: Vec<f32>,
	order: usize,
	input: VecDeque<f32>,
	output: VecDeque<f32>,
}

impl Filter {
	pub fn thiran_allpass(delay: f32, order: usize) -> Self { // https://ccrma.stanford.edu/~jos/pasp/Thiran_Allpass_Interpolators.html
		let mut a = vec![0_f32; order+1];
		let mut b = vec![0_f32; order+1];
		for k in 0..order+1 {
			let mut ak = 1_f32;
			for i in 0..order+1 {
				ak *= (delay-order as f32+i as f32)/(delay-order as f32+k as f32+i as f32);
			}
			let out = (-1_f32).powi(k as i32) * n_choose_k(order, k) as f32 * ak;
			a[k] = out;
			b[order-k] = out;
		}
		let buffer_size = order+1;
		let mut input: VecDeque<f32> = VecDeque::with_capacity(buffer_size);
		let mut output: VecDeque<f32> = VecDeque::with_capacity(buffer_size);
		for i in 0..buffer_size {
			if i == 0 {
				input.push_back(1_f32);
				output.push_back(1_f32);
			} else {
				input.push_back(0_f32);
				output.push_back(0_f32);
			}
		}
		Self {
			a,
			b,
			order,
			input, // x
			output, // y
		}
	}
	
	pub fn _passthru() -> Self {
		let order = 2;
		let buffer_size = order+1;
		let mut input: VecDeque<f32> = VecDeque::with_capacity(buffer_size);
		let mut output: VecDeque<f32> = VecDeque::with_capacity(buffer_size);
		for _i in 0..buffer_size {
			input.push_back(0_f32);
			output.push_back(0_f32);
		}
		Self {
			a: vec![1_f32, 0.5_f32, 0_f32],
			b: vec![1_f32, 0_f32, 0_f32],
			order,
			input, // x
			output, // y
		}
	}
	
	pub fn update(&mut self, input: f32) -> f32 {
		self.input.pop_front();
		self.input.push_back(input);
		
		let mut filtered_output = 0_f32;
		for n in 0..self.order+1 {
			filtered_output += self.b[n]**self.input.get(self.order-n).unwrap();
			if n > 0 {
				filtered_output += self.a[n]**self.output.get(self.order-n).unwrap();
			}
		}
		
		self.output.pop_front();
		self.output.push_back(filtered_output);
		
		filtered_output
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
