const Tipo = {
  sum: "soma",
  multiplication: "multiplication",
  sub: "sub",
  division: "division"
}

function calcularValor(num1, num2, tipoOperecao){
  let numero;
  switch(tipoOperecao){
    case Tipo.sum:
      numero = num1 + num2;
      break;
    case Tipo.multiplication:
      numero = num1 * num2;
      break;
    case Tipo.sub:
      numero = num1 - num2;
      break;
    case Tipo.division:
      numero = num1 / num2;
      break;
    default:
      break;
  }
  return numero
}

function print(firstNumber, lastNumber, calcularValor, calculationType, tipo){
  console.log(`a ${tipo} de ${firstNumber} com ${lastNumber} vai ser igual a ${calcularValor(firstNumber, lastNumber, calculationType)}`)
}

function main(){
  let start = performance.now();
  let firstNumber = 2;
  let lastNumber = 2;
  print(firstNumber, lastNumber, calcularValor, Tipo.sum, "Soma");
  print(firstNumber, lastNumber, calcularValor, Tipo.sub, "Subtracao");
  print(firstNumber, lastNumber, calcularValor, Tipo.division, "Divisao");
  print(firstNumber, lastNumber, calcularValor, Tipo.multiplication, "Multiplicacao");
  let duration = performance.now() - start;
  console.log(`Tempo total: ${duration} ms`);
}
main();
