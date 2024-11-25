use crate::complex::Complex;
use crate::image::Image;
use crate::image::Pixel;

pub fn check_pixel(c: Complex, max_iterations: usize) -> Option<usize> 
{
   let mut z: Complex = Complex{re: 0.0, im: 0.0};

   let mut iteration: usize = 0;
   while iteration < max_iterations
   {
      z = z * z + c;

      if z.mag() > 4.0
      {
         return Some(iteration);
      }

      iteration = iteration + 1;

   }

   return None;
}


//no fucking clue what im supposed to do here ngl
pub fn generate_image(width: usize, height: usize, max_iterations: usize) -> Image 
{

   let mut new_image = Image::new(width, height);

   for x in 0..width
   {
      for y in 0..height
      {
         let cx = (x as f64/new_image.width as f64 - 0.75) * 3.5;
         let cy = (y as f64/new_image.height as f64 - 0.5) * 2.0;
         let c = Complex {re: cx, im: cy};

         if let Some(_) = check_pixel(c, max_iterations) //not in mandelbrot
         {  
            //idk why i need to make if here. the compiler told me to do so
            if let Some(pixel) = new_image.get_mut(x, y)
            {
               *pixel = Pixel {r: 255, g: 255, b:255}
            } 

            
         }
         //else //in mandelbrot
         //dont need to do anything here cuz its already black by default


      }
      
   }

   return new_image;
   

   
}
