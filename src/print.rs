pub fn run() {
    //print to console
    //basic formatting
    println!("{} is {} years ", "angel", "20");

    //positional Arguments
    println!("{0} is from {1} and {0} likes to eat {2}", "angel", "usa", "pizza");


    //Named Arguemens
    println!("{name} likes to play {activity}",name="john", activity="baseball");

    //placeholder traits
    println!("Binary: {:b} Hex: {:x} Octo: {:o}", 10 ,10, 10);

    //placeholder for debub trait
    println!("{:?}",(12, true, "hello"))
}
