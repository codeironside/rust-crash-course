
use std::mem;
pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    //re-assign value
    numbers[2] = 30;

    println!("{:?}", numbers);
    //get single value

    println!("single value {}", numbers[0]);

    //get lenght
    println!("arrays lenth {}", numbers.len());

    //arraysb are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //get slices of arrays

    let slice: &[i32]=&numbers[0..3];
    println!("slice: {:?}", slice);
}
