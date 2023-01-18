
// importing the module 
mod args;
use args::Args;

/*
Rather than doing this to instanciate a new arg, we can create a method that'll do it for us
fn main() {
  let args = Args {
    image_1: String::new(),
    image_2: String::new(),
    output: String::new(),
  };
  println!("Hello, world!");
}
*/



fn main() {
  let args = Args::new();
  println!("{:?}", args);
}



/*
The new()method on String kinda looks like this:

impl String {
  fn new() -> Self {
    String {
      vec: Vec::new(),
    }
  }
}

*/
