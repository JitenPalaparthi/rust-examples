extern crate rand;
use rand::RngCore;
fn main() {
    // get some random data:
    let mut data = [0u8; 20];
    println!("{:?}", data);

    rand::thread_rng().fill_bytes(&mut data);
    println!("{:?}", data);

    let second_largest = get_second_largest(data);
    println!("second largest:{}", second_largest);
}

fn get_second_largest(data: [u8; 20]) -> u8 {
    if data.len() == 1 {
        return data[0];
    }

    if data.len() == 2 {
        if data[0] > data[1] {
            return data[1];
        } else {
            return data[0];
        }
    }

    let mut big: u8 = 0;
    let mut second_big: u8 = 0;

    if data[0] > data[1] {
        big = data[0];
        second_big = data[1];
    } else {
        big = data[1];
        second_big = data[0];
    }

    let mut i: usize = 2;
    while i < data.len() {
        if data[i] >= big {
           second_big=big;
            big = data[i];
        } else if data[i]>second_big{
            second_big=data[i];
        }
        i += 1;
    }

    return second_big;
}
