use std::io;
fn main() {
    println!("Enter a number(integer only)");
    let stdin= io::stdin();
    let mut input:String=String::from("");

    stdin.read_line(&mut input);
    println!("{}",input);
    
    match input.trim().parse::<u32>() {
        Ok(i)=>{
            println!("{} is integer",i);
        },
        Err(_) => println!("invalid input")
    }
    }
