
// importing the module 
mod args;
use std::{io::BufReader, fs::File};
use args::Args;
use image::{io::Reader, ImageFormat, DynamicImage, GenericImageView, imageops::FilterType::Triangle};

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

enum ImageDataErrors {
    DifferentImageFormats,
}

fn main() -> Result<(), ImageDataErrors> { // Handling errors
  let args = Args::new();
  let (image_1, image_format_1) = find_image_from_path(args.image_1);
  let (image_2, image_format_2) = find_image_from_path(args.image_2);
  // println!("{:?}", args);


  if image_1_format != image_format_2 {
    return Err(ImageDataErrors::DifferentImageFormats);
  }

  Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
  let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
  let image_format: ImageFormat = image_reader.format().unwrap();
  let image: DynamicImage= image_reader.decode().unwrap();
  (image, image_format)
}

fn get_smallest_dimension(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
  let pix_1 = dim_1.0 * dim_1.1;
  let pix_2 = dim_2.0 * dim_2.1;
  return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
  let (width, height) = get_smallest_dimension(image_1.dimensions(), image_2.dimensions());
  println!("width: {}, height: {}\n", width, height);

  if image_2.dimensions() == (width, height) {
    (image_1.resize_exact(width, height, Triangle), image_2)
  } else {
      (image_1, image_2.resize_exact(width, height, Triangle))
  }
}
