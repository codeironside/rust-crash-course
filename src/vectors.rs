
use std::mem;
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    //re-assign value
    numbers[2] = 30;

    //add on to vector
    numbers.push(5);
    numbers.push(6);

    //pop off last valued
    numbers.pop();

    println!("{:?}", numbers);
    //get single value

    println!("single value {}", numbers[0]);

    //get lenght
    println!("vector lenth {}", numbers.len());

    //arraysb are stack allocated
    println!("vector occupies {} bytes", mem::size_of_val(&numbers));

    //get slices of arrays

    let slice: &[i32]=&numbers[0..3];
    println!("slice: {:?}", slice);

    //loop through vectors

    for x in numbers.iter() {
        println!("Number: {}", x);
    }


    //loop and mutate values
     for x in numbers.iter_mut() {
        *x *= 2; 
        println!("Numbers vec: {:?}", x);
    }
}
