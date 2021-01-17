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

    // Lopp through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    //  Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // assertion testing
    assert_eq!(2, s.len());
    assert_eq!(11, s.capacity());

    println!("{}", s);


}