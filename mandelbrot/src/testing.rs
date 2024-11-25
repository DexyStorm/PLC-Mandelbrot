


struct Image
{
   width: u16,
   height: u16,
   data: Vec<Pixel>
}

impl Image 
{

   pub fn new(width: usize, height: usize) -> Image
   {
      let data = vec![Pixel{r: 0, g: 0, b: 0}; width*height];

      return Image{width: width as u16, height: height as u16, data};
   }

   pub fn get(&self, x: usize, y: usize) -> Option<&Pixel>
   {
      if x as u16 > self.width
      {
         return None;
      }
      if y as u16 > self.height 
      {
         return None;
      }

      return Some(&self.data[x*y]);
   }

   pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel>
   {
      if x as u16 > self.width
      {
         return None;
      }
      if y as u16 > self.height 
      {
         return None;
      }

      return Some(& mut self.data[x*y]);
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

#[derive(Debug, Copy, Clone, PartialEq)]
struct Pixel
{
   r: u8,
   g: u8,
   b: u8,
}

impl std::fmt::Display for Pixel
{
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
   {
      write!(f, "({}, {}, {})", self.r, self.g, self.b)
   }

}



fn main()
{
   let img1 = Image::new(100, 200);

   let element_0_0 = img1.get(0, 0).unwrap();

   let num: f64 = 5.0;

   let num_pow: f64 = f64::powf(5.0, 2.0);

   println!("{}", num_pow);
}