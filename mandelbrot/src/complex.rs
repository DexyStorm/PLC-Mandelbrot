// implement the Complex struct and traits below

#[derive(Copy, Clone)]
pub struct Complex
{
   pub re: f64,
   pub im: f64
}


impl std::ops::Add for Complex
{
   type Output = Complex;

   fn add(self, other: Complex) -> Complex
   {
      Complex
      {
         re: self.re + other.re,
         im: self.im + other.im,
      }
   }
}

impl std::ops::Mul for Complex
{
   type Output = Complex;

   fn mul(self, other: Complex) -> Complex
   {
      Complex
      {
         re: (self.re * other.re - self.im * other.im),
         im: (self.re * other.im + self.im * other.re),
      }
   }
}  

impl Complex
{

   pub fn mag(self) -> f64
   {
      return f64::powf(self.re, 2.0) + (f64::powf(self.im, 2.0));
      
   }
}