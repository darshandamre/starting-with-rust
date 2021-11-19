// looops are used to iterate until a condition is met

pub fn run() {
  let mut count = 1;
  let mut count2 = 1;

  // infinite loop
  loop {
    count2 += 1;
    println!("Number: {}", count2);

    if count2 == 20 {
      break;
    }
  }

  // while loop (FizzBuzz)
  while count <= 100 {
    if count % 15 == 0 {
      println!("FizzBuzz");
    } else if count % 3 == 0 {
      println!("Fizz");
    } else if count % 5 == 0 {
      println!("Buzz");
    } else {
      println!("{}", count);
    }

    // increment
    count += 1;
  }

  // for range loop
  for x in 0..100 {
    if x % 15 == 0 {
      println!("FizzBuzz");
    } else if x % 3 == 0 {
      println!("Fizz");
    } else if x % 5 == 0 {
      println!("Buzz");
    } else {
      println!("{}", x);
    }
  }
}
