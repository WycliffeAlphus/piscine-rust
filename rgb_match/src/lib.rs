#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    #[allow(unused_mut)]
    pub fn swap(mut self, first: u8, second: u8) -> Color {

       fn swap_value(v: u8, first: u8, second: u8) -> u8 {
        match v {
            x if x == first => second,
            x if x == second => first,
            _=>v,
        }
       }


       Color {
        r: swap_value(self.r, first, second),
        g: swap_value(self.g, first, second),
        b: swap_value(self.b, first, second),
        a: swap_value(self.a, first, second),
       }
    }
} 
