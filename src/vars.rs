//Variables hold primitive data types or references to data
//Variables are immutable by default
//Rust is a block-scoped language

pub fn run(){
    let name = "Yuki";
    let mut age = 23; //mut => allows to change var (immutable!)

    println!("My name is {} and i'm {}", name, age);

    age = 24;

    println!("My name is {} and i'm {}", name, age);

    //Define Constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple Vars
    let (my_name, my_age) = ("Yuki", 23);
    println!("{} is {}", my_name, my_age);
}