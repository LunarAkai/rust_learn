pub fn run(){
    //print to console
    println!("Hello from print.rs");

    //Basic Formatting
    println!("{} likes {}", "Yuki", "Cake");

    //Positional Args
    println!("{0} likes {1} and {0} likes to eat {2}", "Yuki", "Cake", "Strawberry");

    //Named Args
    println!("{name} likes to play {activity}", name = "Yuki", activity = "Chess");

    //Placeholder traits
    println!("Binary {:b} Hex: {:x}, Octal {:o}", 10, 10, 10 );

    //Place holder for Debug trait
    println!("{:?}", (12, true, "hello"));

    //basic math
    println!("10 + 10 = {}", 10 + 10);
}