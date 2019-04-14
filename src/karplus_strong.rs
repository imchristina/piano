use std::collections::VecDeque;

pub struct KarplusStrong {
	pub delay: VecDeque<f32>,
	pub previous_sample: f32,
}

impl KarplusStrong {
	pub fn update(&mut self) -> f32 {
		let sample = match self.delay.pop_back() {
			Some(v) => v,
			None => 0_f32,
		};
		let avg = (sample+self.previous_sample)/2_f32; // TODO bad unwrap, do match instead
		self.delay.push_front(avg);
		self.previous_sample = avg;
		avg
	}
}
