fn main() {
   
let  mut r1=Rect{l:10.,b:20.,a:0f32};
let s1:Square=Square{s:25.};
area(Box::new(&r1));
r1.area();
println!("A:{}",r1.a);

}

fn area(t:Box<&dyn Area>){
  let a1 = t.area();
  println!("Area:{}",a1)
}


trait Area{
    fn area(&mut self)->f32;
}

struct Rect{
    l:f32,
    b:f32,
    a:f32,
}

struct Square{
    s:f32
}

impl Area for Rect{
    fn area(&mut self)->f32 {
        self.a=self.l*self.b;
        return self.a;
    }
}

impl Area for Square{
    fn area(&mut self)->f32 {
        self.s * self.s
    }
}