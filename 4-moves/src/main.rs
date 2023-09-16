fn main() {
    println!("Hello, world!");

    let arr1 = [1,2,3,4,5];
    println!("Sum of arr:{}",sum_of(arr1));
    println!("Sum of arr:{}",sum1_of(arr1));
    println!("Sum of arr:{}",sum2_of(arr1));
    println!("Sum of slice:{}",sum_of_slice(&arr1));
}

fn sum_of(arr:[i32;5])->i32{
let mut sum=0;
    for i in &arr {
        sum+=i;
    }
    sum
}


fn sum_of_slice(arr: &[i32])->i32{
    let mut sum=0;
        for i in arr {
            sum+=i;
        }
        sum
    }

fn sum1_of(arr:[i32;5])->i32{
    let mut sum=0;
        for i in arr.iter() {
            sum+=i;
        }
        sum
    }

    fn sum2_of(arr:[i32;5])->i32{
        let mut i=0;
        let mut sum =0;
        while i<arr.len(){
            sum+=arr[i];
            i=i+1;
        }
        sum
    }
       

