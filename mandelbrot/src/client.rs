use std::num::ParseIntError;
use std::env;

use std::fs;
// you may need use std::env for parsing arguments
// use std::num::ParseIntError;

use crate::image::Image; // use from another module

// uncomment and implement: 
pub fn parse_args() -> Result<(usize, usize, usize), ParseIntError> 
{
	let args: Vec<String> = env::args().collect();

	if args.len() != 4
	{
		println!("Usage: {} <width> <height> <max_iterations>", args[0]);
		std::process::exit(1);
	}

	let arg_1: usize = args[1].parse().unwrap();
	let arg_2: usize = args[2].parse().unwrap();
	let arg_3: usize = args[3].parse().unwrap();

	return Ok((arg_1, arg_2, arg_3));
}


pub fn save_to_file(image: &Image, filename: &str) {
	let mut s = String::new();
	s.push_str(&format!("P3\n{} {}\n255\n", image.width, image.height));
	
	for y in 0..image.height {
		for x in 0..image.width {
			let pixel = image.get(x, y).unwrap();
			s.push_str(&format!("{}\n", pixel));
		}
	}

	fs::write(filename, s).expect("Error writing to disk!");
}
