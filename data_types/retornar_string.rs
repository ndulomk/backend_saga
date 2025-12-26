// retornando uma string literal fixa:

fn last_name() -> &'static str {
    "Janota"
}


// Se o valor pode ser dinâmico:

fn last_name() -> String {
    "Janota".to_string()
}

// Ou

fn last_name() -> String {
    String::from("Janota")
}


// retornar referência ligada a um input
fn last_name<'a>(input: &'a str) -> &'a str {
    input
}
