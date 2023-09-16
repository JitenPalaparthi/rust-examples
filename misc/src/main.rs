use std::any::type_name;

#[warn(unused_variables)]
fn main() {
    println!("Hello, world!");
    let what_is_this=();

    println!("{:#?}",what_is_this);

    let num1 = 1001;

    println!("{}", type_of(num1));

   let str1 = "Hello World";
   

   let str2= str1.to_string();


println!("{}",str1);

}

fn type_of<T>(_: T) -> &'static str{
    type_name::<T>()
}