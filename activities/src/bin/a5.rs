// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn display_with_loop(){
  let mut a: i32 = 1;
  loop {
    if  a == 6 {
      break;
    }

    println!("{:?}", a);
    a = a + 1;
  }
}

fn display_with_while(){
  let mut number: i32 = 1;
  while number != 6 {
    println!("{number}");
    number = number + 1;
  }
}

fn main() {
  display_with_loop()
}
