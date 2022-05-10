// Vectors - Resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    //get single value
    println!("Single Value: {}", numbers[0]);

    //Reassign value
    numbers[2] = 20;

    //Add on to vector
    numbers.push(6);
    numbers.push(7);

    //pop of last value
    numbers.pop();

    println!("{:?}", numbers);

    //get vector length
    println!("Vector length: {}", numbers.len());

    //vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    //loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    //loop and mutate value
    for x in numbers.iter_mut() {
        *x *=2; //multiply every value by 2
    }

    println!("Numbers Vec: {:?}", numbers);
}