fn main() {
    
    let s1="Hello";
    let s2 ="World";
    let s3:String= format!("{} {}",s1,s2);
    println!("{}",s3);

    let s4="Hello";
    let s5:String="World".to_string();
 
    let s6:&str=s5.as_str(); //string to str



}

//https://stackoverflow.com/questions/30154541/how-do-i-concatenate-strings
/*
str and str
String and str
String and String
*/