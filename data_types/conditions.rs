let mut a = 99;
if a > 99 {
  println!("Big number");
} else {
  println!("Small number");
}


//nested

a = 99;

if a > 99 {
  if a > 200 {
    println!("Huge Number");
  } else {
    println!("Big Number");
  }
} else {
  println!("Small number");
}