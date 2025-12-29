/*
THE PRINTLN MACRO

*Macros expand into additional code
*println "Prints" (displays) information to the terminal
*Useful for debugging
*/

// static life: i32 = 42;
// println!("hello");
// println!("{:?}", life); 
// println!("{:?} {:?}", life, life);
// println!("the meaning is {:?}", life)




// {:?} is for debugging without it is just for showing uses

enum TipoOperacao {
  SOMA,
  DIVISAO,
  MULTIPLICACAO,
  SUBTRACAO
}


fn calculation(num1: i32, num2: i32, calculation: TipoOperacao) -> i32 {
  match calculation {
    TipoOperacao::SOMA => num1 + num2,
    TipoOperacao::DIVISAO => num1 / num2,
    TipoOperacao::MULTIPLICACAO => num1 * num2,
    TipoOperacao::SUBTRACAO => num1 - num2,
  }
}

fn main(){
  let sum: i32 = calculation(2, 2, TipoOperacao::SOMA);
  let value = calculation(10, 5, TipoOperacao::SUBTRACAO);
  let division = calculation(10, 2, TipoOperacao::DIVISAO);
  let mult = calculation(5, 5, TipoOperacao::MULTIPLICACAO);
  println!("Soma = {sum}, Sub = {value}, division = {division}, mult = {mult}")
}