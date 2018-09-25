use waveguide::Waveguide;
extern crate rand;

pub fn hammer(string: &mut Waveguide, stiffness: f32, size: usize, velocity: f32, position: usize, time: f32) {
	let length_strike = 0.00;
	let length_retract = 0.01;
	let length_total = length_strike + length_retract;
	
	if time < length_total && position+size < string.length {
		let mut hammer_position;
		if time < length_retract {
			hammer_position = 1_f32;//time/length_strike;
		} else {
			hammer_position = 1_f32-(time-length_strike)/length_retract;
		}
		
		hammer_position *= velocity/10_f32;
		
		for i in position..position+size {
			let y = string.get_displacement(i);
			if y < hammer_position {
				string.set_displacement(i, y + ((hammer_position-y)*stiffness));
			}
		}
	}
}

pub fn rand_hammer(string: &mut Waveguide, velocity: f32, time: f32) {
	if time == 0.0 {
		for i in 1..string.length {
			string.set_displacement(i, rand::random::<f32>()*velocity);
		}
	}
}

pub fn damper(string: &mut Waveguide, stiffness: f32, size: usize, position: usize, time: f32) -> bool {
	let length_strike = 0.08;

	for i in position..position+size {
		let y = string.get_displacement(i);
		string.set_displacement(i, y*stiffness);
	}
	time > length_strike
}

