// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

trait Label {
  fn print(&self);
}

struct One;
struct Two;
struct Three;
struct Other;

impl Label for One { fn print(&self) { println!("One"); } }
impl Label for Two { fn print(&self) { println!("two"); } }
impl Label for Three { fn print(&self) { println!("three") } }

impl Label for Other { fn print(&self) { println!("other"); } }

fn get_label(x: i32) -> Box<dyn Label> {
  match x {
      1 => Box::new(One),
      2 => Box::new(Two),
      3 => Box::new(Three),
      _=> Box::new(Other)
  }
}

fn main() {
  get_label(3).print();
}
