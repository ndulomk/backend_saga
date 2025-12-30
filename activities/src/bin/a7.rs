// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print


enum COLOR {
  AZUL,
  VERMELHO,
  BRANCO, 
  VERDE
}

fn print_color(color: COLOR){
  match color {
    COLOR::AZUL => println!("Azul"),
    COLOR::VERMELHO => println!("Vermelho"),
    COLOR::BRANCO => println!("Branco"),
    COLOR::VERDE => println!("Verde"), 
  }
}

fn main() {
  print_color(COLOR::AZUL);
  print_color(COLOR::VERMELHO);
  print_color(COLOR::BRANCO);
  print_color(COLOR::VERDE);
}
