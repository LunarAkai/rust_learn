// Conditionals - Used to check the condition of something and act on the result

pub fn run() {
    let age:u8 = 18;
    let check_id = true;

    //if else
    if age >= 18 && check_id {
        println!("You're an adult!");
    } else if age < 18 && check_id {
        println!("You're not an adult")
    } else {
        println!("I need to see your id");
    }
}