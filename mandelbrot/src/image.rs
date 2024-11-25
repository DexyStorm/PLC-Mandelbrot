// implement the Pixel struct and traits below

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pixel
{
   pub r: u8,
   pub g: u8,
   pub b: u8,
}

impl std::fmt::Display for Pixel
{
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
   {
      write!(f, "{} {} {}", self.r, self.g, self.b)
   }

}

pub struct Image
{
   pub width: usize,
   pub height: usize,
   pub data: Vec<Pixel>
}

impl Image 
{

   pub fn new(width: usize, height: usize) -> Image
   {
      let data = vec![Pixel{r: 0, g: 0, b: 0}; width*height];

      return Image{width: width, height: height, data};
   }

   pub fn get(&self, x: usize, y: usize) -> Option<&Pixel>
   {
      if x > self.width
      {
         return None;
      }
      if y > self.height 
      {
         return None;
      }

      return Some(&self.data[y * self.width + x]);
   }

   pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel>
   {
      if x > self.width
      {
         return None;
      }
      if y > self.height 
      {
         return None;
      }

      return Some(& mut self.data[y * self.width + x]);
   }

   pub fn get_mandelbrot_pixels(&self) -> usize
   {
      let count = self.data.iter().filter (|x|
      {
         if (x.r == 0) && (x.g == 0) && (x.b == 0)
         {
            return true;
         }
         return false;
      }).count();

      return count;
   }

}