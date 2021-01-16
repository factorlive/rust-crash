pub fn run() {
    let name = "Patrick";
    let mut age = 34;

    println!("My name is {} and I am {}", name, age);

    age = 38;
    
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let ( my_name, my_age) = ("Patrick", 34);
    println!("My name is {} and I am {}", my_name, my_age);
}