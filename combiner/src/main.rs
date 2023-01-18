
// importing the module 
mod args;
use std::{io::BufReader, fs::File};
use args::Args;
use image::{io::Reader, ImageFormat, DynamicImage};

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
  let (image_1, image_format_1) = find_image_from_path(args.image_1);
  let (image_2, image_format_2) = find_image_from_path(args.image_2);
  // println!("{:?}", args);
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
  let image_format: ImageFormat = image_reader.format().unwrap();
  let image: DynamicImage= image_reader.decode().unwrap();
  (image, image_format)
}
