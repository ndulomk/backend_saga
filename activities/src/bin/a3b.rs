// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn dealing_with_five(variable: i32){
  let my_number: i32 = 5;
  if variable > my_number {
    println!(">{my_number}");
    return 
  }
  if variable < my_number {
    println!("<{my_number}");
    return
  }
  println!("={my_number}");
}


fn main() {
  let variable: i32 = 5;
  dealing_with_five(variable);
}
