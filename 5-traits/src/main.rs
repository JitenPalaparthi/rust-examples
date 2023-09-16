fn main() {

    let t1:Box<dyn Area>=Box::new(Square::new(25.25));
    t1.area();
    t1.print();
}

trait Area {
    fn area(&self) -> f32;
    fn who(&self) {
        println!("I am Area trait");
    }
    fn print(&self);
}

struct Square(f32);

impl Square{
    fn new(s:f32)->Self{
        Self(s)
    }
}

impl Area for Square{
   fn area(&self) -> f32 {
       return self.0 * self.0
   }
   fn print(&self) {
       println!("Area of Square:{:.2}",self.0*self.0);
   }
}

// creating a traint object?
