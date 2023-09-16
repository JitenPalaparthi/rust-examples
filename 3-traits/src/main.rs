fn main(){
    println!("Hello Muruga!");

    let r1:Rect=Rect{l:12.4,b:13.5};
    let s1:Square=Square(25.25);

    // println!("Area of Rect r1:{}",r1.area());
    // println!("Area of Square s1:{:.2}",s1.area());
    area(&r1);
    area(&s1);

    println!("Area of Rect r1:{}",r1.area());
    println!("Area of Square s1:{:.2}",s1.area());

}


fn area(a:&impl Area){
     println!("area:{:.2}",a.area());
}

trait Area{
    fn area(&self)->f32;
}

struct Rect{
    l:f32,
    b:f32,
}


impl Area for Rect{
    fn area(&self)->f32 {
        return self.b * self.l;
    }
}

struct Square(f32);

impl Area for Square{
    fn area(&self)->f32 {
        self.0 * self .0
    }
}