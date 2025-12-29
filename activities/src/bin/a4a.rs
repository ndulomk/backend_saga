// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn true_or_false(variable: bool){
  if variable == false {
    println!("It's false");
    return
  }
  println!("It's true")

}

fn main() {
  true_or_false(false)
}
