// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    //get single value
    println!("Single Value: {}", numbers[0]);

    //Reassign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    //get array length
    println!("Array length: {}", numbers.len());

    //arrays are stack allocated
    println!("Arrays occupies {} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}