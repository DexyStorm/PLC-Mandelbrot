use std::fs;
// you may need use std::env for parsing arguments
// use std::num::ParseIntError;

use crate::image::Image; // use from another module

// uncomment and implement: 
// pub fn parse_args() -> Result<(usize, usize, usize), ParseIntError> {
    // parse input arguments here, and return their values
    
// }


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
