use string;

pub fn hammer(string: &mut string::String, stiffness: f32, size: usize, velocity: f32, position: usize, time: f32, dt: f32) { // time is now in seconds
	let length_strike = 0.01;
	let length_retract = 0.01;
	let length_total = length_strike + length_retract;
	let mut hammer_position = -10_f32;
	
	if time < length_total {
		if time < length_retract {
			hammer_position = time/length_strike;
		} else {
			hammer_position = 1_f32-(time-length_strike)/length_retract;
		}
		
		hammer_position *= velocity/5_f32;
		
		for i in position..position+size {
			if string.y[i] < hammer_position {
				string.y[i] += (hammer_position-string.y[i])*stiffness;
			}
		}
	}
}
