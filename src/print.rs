pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    
    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Place holder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
 
    println!("10 + 10 = {}", 10 + 10);
}