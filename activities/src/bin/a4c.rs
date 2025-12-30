// Se ele pode votar

trait Ages {
  fn print(&self);
}

struct Dezoito;
struct Dezasseis;
struct Vintedois;

struct Acimadevintedois;
struct Abaixodequinze;

impl Ages for Dezoito { fn print(&self) {
    println!("Tem idade pra votar")
} }

impl Ages for Dezasseis { fn print(&self) { println!("Espere mais alguns anos") } }

impl Ages for Vintedois { fn print(&self) { println!("Mata lil bro, ja podes votar") } }

impl Ages for Acimadevintedois { fn print(&self) { println!("Faz ja filho e casa ja maluco") } }

impl Ages for Abaixodequinze { fn print(&self) { println!("Vai tomar leite dumbass kid") } }

fn get_age(age: i32) -> Box<dyn Ages> {
  match age {
    16 => Box::new(Dezasseis),
    18 => Box::new(Dezoito),
    22 => Box::new(Vintedois),  
    age if age > 18 => Box::new(Acimadevintedois),
    _=> Box::new(Abaixodequinze)

  }
}


fn main(){
  get_age(20).print();
}

