fn main() {
    println!("Hello, world!");

    let mut c1: C = C { sum: 0 };
    let v1 = vec![10, 12, 13];
    let v2: Vec<i32> = vec![10];
    let a = c1.add(v1).add(v2).add(vec![10,20]).get();
    println!("{}", a)
}

// trait Area{

// }

trait Calculate {
    fn add(&mut self, v: Vec<i32>) -> Box<&mut dyn Calculate>;
    fn get(&mut self) -> i32;
}

struct C {
    sum: i32,
}

impl Calculate for C {
    fn add(&mut self, v: Vec<i32>) -> Box<&mut dyn Calculate> {
        for n in v {
            self.sum = self.sum + n;
        }
        return Box::new(self);
    }

    fn get(&mut self) -> i32 {
        return self.sum;
    }
}
