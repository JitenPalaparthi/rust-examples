use std;
fn main() {
    let str1= "Hello World";
    print_type(str1); // type str

    let str2 = str1;

    print_type(str2);

    let string1= String::from("Hello World");
    print_type(&string1);

    let string2 = &string1; // cannot be used after a move

    print_type(string2);

    let st=getString(string2);

    println!("{}",st);


}

fn getString(st:str)->str{
return st;
}

use std::mem;

fn print_type<T: ?Sized>(_: &T) {
    println!("Type: {}", std::any::type_name::<T>());
}