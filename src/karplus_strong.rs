use std::collections::VecDeque;

pub struct KarplusStrong {
	pub average_weight: f32,
	pub delay: VecDeque<f32>,
	pub previous_sample: f32,
}

impl KarplusStrong {
	pub fn update(&mut self) -> f32 {
		let sample = match self.delay.pop_back() {
			Some(v) => v,
			None => 0_f32,
		};
		let avg = (sample+self.previous_sample*self.average_weight)/(1_f32+self.average_weight);
		self.delay.push_front(avg);
		self.previous_sample = avg;
		avg
	}
}
