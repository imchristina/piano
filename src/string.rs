pub struct String {
	pub length: usize, // number of points in string
	pub dispersion: f32, // coefficient of energy transfered into surrounding points, does not introduce loss
	pub loss: f32,
	pub termination_points: usize,
	pub termination_force: f32,
	pub y: Vec<f32>,
	pub v: Vec<f32>
}

pub fn new(length: usize, dispersion: f32, loss: f32, termination_points: usize) -> String {
	let y: Vec<f32> = vec![0_f32; length];
	let v: Vec<f32> = vec![0_f32; length];
	let termination_force = 1.0_f32/termination_points as f32;
	String {
		length: length-1, // make length zero-indexed
		dispersion,
		loss,
		termination_points,
		termination_force,
		y,
		v
	}
}

pub fn update(s: &mut String) -> (f32, f32) {
	for i in 0..s.length { // calculate forces
		let energy = (s.y[i]-s.y[i+1])*s.dispersion;
		s.v[i+1] = (s.v[i+1] + energy) * s.loss;
		s.v[i] -= energy;
	}
	for i in 0..s.length { // apply forces
		s.y[i] = s.y[i] + s.v[i]; // might be better to do loss here
	}
	for i in 0..s.termination_points { // soft terminations
		s.y[i] *= s.termination_force*(i as f32); // left
		s.y[(s.length-s.termination_points)+i] *= 1_f32-(s.termination_force*i as f32); // right
	}
	let force_left = s.v[0];
	let force_right = s.v[s.length];
	s.y[0] = 0_f32; // rigid terminations
	s.y[s.length] = 0_f32;
	s.v[0] = 0_f32;
	s.v[s.length] = 0_f32;
	(force_left, force_right) // return v forces at termination points
}

#[cfg(test)]
mod tests {
	use string;
    #[test]
    fn test_string_setup() {
        let mut string = string::new(100, 1_f32, 1_f32, 3);
    }
	#[test]
	fn test_string_update() {
		let mut string = string::new(100, 1_f32, 1_f32, 3);
		for i in 0..10000 {
			string::update(&mut string);
		}
		let mut maxn = 0_f32;
		for value in string.y {
			if value > maxn {
				maxn = value;
			}
		}
		for value in string.v {
			if value > maxn {
				maxn = value;
			}
		}
		assert_eq!(maxn < 1_f32, true); // make sure string doesn't explode
		assert_eq!(maxn > -1_f32, true);
	}
}
