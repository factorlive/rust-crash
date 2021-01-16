pub fn run() {
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());
     
    hello.push('W');

    hello.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    println!("Is empty: {}", hello.is_empty());

    println!("{}", hello);
}