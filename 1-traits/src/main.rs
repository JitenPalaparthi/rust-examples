use std::fmt;

fn main() {
    println!("Hello, world!");
    {
    let r1=Rect{l:10.,b:12.};
    println!("{}",r1);
    }
    let e1 = empty{};

    println!("{}",e1);

    let a1 = Rect{l:10.,b:20.};
    //let a2:dyn Area = a1 as dyn Area;


}

trait Area {
    fn area(&self)->f32;
}

trait Perimeter{
    fn perimeter(&self)->f32;
}

struct Rect {
    l: f32,
    b: f32,
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Length:{} Bredth:{}", self.l, self.b)
    }
}

impl Drop for Rect{
    fn drop(&mut self) {
        println!("I am dropping from the memory");
    }
   
}
impl Area for Rect{
    fn area(&self)->f32{
        self.l*self.b
    }
}




struct empty;

impl std::fmt::Display for empty{

fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f,"")
}
}

