extern crate vst;
mod string;

fn main() {
	let mut string = string::new(500, 1_f32, 1_f32, 3);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
