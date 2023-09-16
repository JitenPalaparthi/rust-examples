fn main() {
    let mut hello:&str ="Hello World";
    hello = "Hello World !";
    println!("{}",hello);

    let str1:String= hello.to_string();

    println!("{}",str1);

    let str2:&str=&str1;

    let ch= str1.chars().nth(1).unwrap();

    println!("{:?}",ch);

    let bs= str1.as_bytes();
    println!("{:?}",bs.len());

    let c1= get_length(&str1);
    let c2=get_length(str2);
    let form=format!("length of str1:{} and length of str2:{}",c1,c2);

    println!("{}",form);

    let s1="Hello".to_string();

    let s2 ="World";

    let s3 = s1+s2;

    println!("{}",s3);

}

fn get_length(str1:&str)->i32{

let len= str1.len();

    len as i32
}