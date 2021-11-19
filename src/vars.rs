// variables hold primitive data or references to data
// variables are immutable by default
// rust is a block scoped language

pub fn run() {
  let name = "Darshan";
  let mut age = 19;
  age += 1;
  println!("My name is {} and i am {}", name, age);

  // define constant
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // assign multiple vars
  let (my_name, my_age) = ("darshan", 19);
  println!("{} is {}", my_name, my_age)
}