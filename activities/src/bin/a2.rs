use std::time::Instant;
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
fn print(first_number: i32, last_number: i32, calcular_valor: fn(i32, i32, Tipo) -> i32, calculation_type: Tipo, tipo: &str){
  println!("a {tipo} de {first_number} com {last_number} vai ser igual a {:?}", calcular_valor(first_number, last_number, calculation_type))
}
fn main() {
  let start: Instant = Instant::now();
  let first_number: i32 = 2;
  let last_number: i32 = 2;
  print(first_number, last_number, calcular_valor, Tipo::Sum,"Soma");
  print(first_number, last_number, calcular_valor, Tipo::Sub,"subtracao");
  print(first_number, last_number, calcular_valor, Tipo::Division,"divisao");
  print(first_number, last_number, calcular_valor, Tipo::Multiplication,"multiplicacao");
  let duration = start.elapsed();
  println!("Tempo pra executar o codigo: {:?}", duration);
}
