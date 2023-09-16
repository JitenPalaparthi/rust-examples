fn main() {
    println!("Hello, world!");
    fibonachi(10);
}

fn fibonachi(_n: i32) {
    let mut a = 0;
    let mut b = 1;
    print!("{},{}", a, b);
    //a,b=10,20;
    let mut i = 2;
    while i <= _n {
        let mut c = a + b;
        a = b;
        b = c;
        print!("{},", c);
        i += 1;
    }
}
