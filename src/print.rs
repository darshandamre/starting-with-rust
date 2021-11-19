pub fn run() {
  // print to console
  println!("Hello from the print.rs file");

  // Basic Formatting
  println!("{} is from {}", "Darshan", "Mumbai");

  // Positional arguments
  println!("{0} is from {1} and {0} likes to {2}", "Darshan", "Mumbai", "code");

  // named arguments
  println!("{name} likes to play {activity}", name = "John", activity = "baseball");

  // placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

  // placeholder for debug trait
  println!("{:?}", (12, true, "Hello"));

  // basic math
  println!("10 + 10 = {}", 10 + 10);
}