// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

enum Tipo {
  Sum,
  Multiplication,
  Sub,
  Division,
}
fn calcular_valor(num1: i32, num2:i32, tipo_operacao: Tipo) -> i32 {
  match tipo_operacao {
    Tipo::Sum => return num1 + num2,
    Tipo::Multiplication => return num1 * num2,
    Tipo::Sub => return num1 - num2,
    Tipo::Division => return num1 / num2 
  }
}
fn main() {
  let first_number: i32 = 2;
  let last_number: i32 = 2;
  println!("A soma de {first_number} + {last_number} = {:?}", calcular_valor(first_number, last_number, Tipo::Sum));
  println!("A subtracao de {first_number} + {last_number} = {:?}", calcular_valor(first_number, last_number, Tipo::Sub));
  println!("A divisao de {first_number} + {last_number} = {:?}", calcular_valor(first_number, last_number, Tipo::Division));
  println!("A multiplicacao de {first_number} + {last_number} = {:?}", calcular_valor(first_number, last_number, Tipo::Multiplication))
}
