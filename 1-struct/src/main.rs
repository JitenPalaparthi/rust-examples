fn main() {
    println!("Hello, world!");
    let mut r1= Rect::new(10.,12.);
   println!("Area of rect after calling the function:{}", r1.area());
    println!("Area of rect inside r1:{}",r1.a);
    
}

// trait Area {
//     fn area(&self) -> f32;
// }

struct Rect {
    l: f32,
    b: f32,
    a: f32,
}

impl Rect {
    fn new(l: f32, b: f32) ->Self {
        Self { l: l, b: b, a: 0. }
    }
    fn area(&mut self) -> f32 {
        self.a = self.l * self.b;
        self.a
    }
}
