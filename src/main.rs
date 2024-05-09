mod array_generator;

fn main() {
    let mut array = array_generator::array_generator::<u8>();
    bubbelsort(&mut array);
    for x in array {
        print!("{},", x);
    }
    println!();
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
