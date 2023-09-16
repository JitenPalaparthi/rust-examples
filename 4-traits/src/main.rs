fn main() {
    println!("Hello, world!");
let r1 = Rect::new(10.,12.);
println!("{:.2}",r1.area());
area(&r1);
}


fn area<R:Area+Perimeter>(a:&R){
println!("Area of Rect:{}",a.area());
println!("Perimeter of Rect:{}",a.perimeter());

}

trait Area{
    fn area(&self)->f32;
}

trait Perimeter{
    fn perimeter(&self)->f32;
}

struct Rect{
    l:f32,
    b:f32,
}

impl Rect{
    fn new(l:f32,b:f32)->Self{
        Self { l: l, b: b }
    }
}

struct Square(f32);

impl Area for Rect{
    fn area(&self)->f32 {
        self.l * self.b
    }
}

impl Perimeter for Rect{
    fn perimeter(&self)->f32 {
        2.0*(self.l +self.b)
    }
}


impl Area for Square{
    fn area(&self)->f32 {
        self.0 * self.0
    }
}

// trait bound sytax
