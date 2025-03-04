//prinitive string
//string=growable

pub fn run() {
    let hello = "Hello";
    let mut world = String::from("world");

    //get length
    world.push('W');

    world.push_str("ealth");

    println!("length is: {}", world.len());

    println!("word  is: {}", world);

    //get caopacity in bytes

    println!("capacit :{}", world.capacity());

    //is empty

    println!("is empty :{}", world.is_empty());

    //contains
    println!("contains worl  d :{}", world.contains("world"));

    //replace
    println!("replace :{}", world.replace("world","there"));

    //loop through string by whitespce
    for word in hello.split_whitespace(){
        println!("{}",word);
    }

    //create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    //Assertion testing

    assert_eq!(3,s.len());
    assert_eq!(11,s.len());
}
