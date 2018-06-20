use string;

pub fn hammer(string: &mut string::String, stiffness: f32, size: usize, velocity: f32, position: usize, time: usize) {
	let t_time = (1000_f32/velocity) as usize;
	if time < t_time {
		let v_pos = 1_f32/(t_time-time) as f32;
		for i in position..position+size {
			if string.y[i] < v_pos {
				string.y[i] += (v_pos-string.y[i])*stiffness;
			}
		}
	} /*else if time < t_time*2 {
		let v_pos = 1_f32/(t_time-time) as f32;
		for i in position..position+size {
			if string.y[i] < v_pos {
				string.y[i] += (v_pos-string.y[i])*stiffness;
			}
		}
	}*/
}
