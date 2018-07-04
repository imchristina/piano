pub struct String {
	length: usize,
	r: Vec<f32>,
	l: Vec<f32>,
	pointer: usize, // end of string, +1 is start of string
}

pub fn new(length: usize) -> String {
	String {
		length: length - 1,
		r: vec![0_f32, length],
		l: vec![0_f32, length],
		pointer: 0,
	}
}

pub fn update(string: &mut String) -> (f32, f32) {
	let end_r = string.l[string.pointer];
	let end_l = string.r[string.pointer];
	// do termination dispersion stuff here if neccesary (?)
	string.r[string.pointer+1] = -end_l;
	string.l[string.pointer+1] = -end_r;
	
	string.pointer += 1;
	if string.pointer >= string.length {
		string.pointer = 0;
	}
	
	(end_r, end_l)
}
