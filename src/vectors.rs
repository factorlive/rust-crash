
use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // Re-assign value
    numbers[2] = 20;

    // pushing and pop off last item
    numbers.push(5);

    numbers.push(6);

    numbers.pop();
    println!("{:?}", numbers);

    // Get single value
    println!("Single Value: {}", numbers[0]);

    // Get array length
    println!("Vector Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[2..4];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }    

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec after mut: {:?}", numbers);
}