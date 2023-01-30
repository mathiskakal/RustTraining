// Handling the arguments and their parsing a little better in their own module
fn get_nth_arg(n: usize) -> String {
  std::env::args().nth(n).unwrap()
}


//use of pub keyword to make it accessible outside of the urrent module
#[derive(Debug)] // this is an attribute deriving the debug trait for the struct which will allow us to print it
pub struct Args {
  pub image_1: String,
  pub image_2: String,
  pub output: String
}

// Args::new() method
impl Args {
  pub fn new() -> Self {
    Args {
    image_1: get_nth_arg(1),
    image_2: get_nth_arg(2),
    output: get_nth_arg(3),
    }
  }
}