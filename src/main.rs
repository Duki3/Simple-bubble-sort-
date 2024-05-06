use rand::{distributions::Standard, prelude::*};
fn main() {
    let mut array = array_generator::<i8>();
    bubbelsort(&mut array);
    for x in array {
        print!("{},", x);
    }
    println!();
}

fn array_generator<T>() -> Vec<T> 
where Standard:Distribution<T>{
    let mut buffer = Vec::new();
    for _ in 0..random::<u8>() {
        buffer.push(random::<T>());
    }
    buffer
}

fn bubbelsort<T>(array: &mut [T])
where
    T: PartialOrd,
{
    for n in (2..=array.len()).rev() {
        for i in 0..n - 1 {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
            }
        }
    }
}
