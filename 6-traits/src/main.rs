fn main() {
    let s1=sq(100);

    let s2=sq(100);

    let s3 = sq(300).clone();

    if s1==s2{
        println!("both of them are equal");
    }

    println!("{:?}",s3);
}

// derive .

#[derive(PartialEq,Clone,Debug)]
struct sq(i32);



