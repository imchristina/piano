use string;

pub fn hammer(string: &mut string::String, stiffness: f32, size: usize, velocity: f32, position: usize, time: f32) {
	let length_strike = 0.00;
	let length_retract = 1.11;
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
				string.set_displacement(i, y + (hammer_position-y)*stiffness);
			}
		}
	}
}

pub fn damper(string: &mut string::String, stiffness: f32, size: usize, position: usize, time: f32) -> bool {
	let length_strike = 0.05;
	let damper_position;
	
	if time < length_strike && position+size < string.length {
		damper_position = (time/length_strike)-1_f32;
	} else {
		damper_position = 1_f32;
	}
		
	for i in position..position+size {
			let y = string.get_displacement(i);
			if y < damper_position {
				string.set_displacement(i, y + (damper_position-y)*stiffness);
			}
		}
	time > 4_f32*length_strike
}
