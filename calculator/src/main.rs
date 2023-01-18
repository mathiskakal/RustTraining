// default entrypoint to the application

use std::env::{args, Args};

fn main() {
  let mut args: Args = args();

  let first = args.nth(1).unwrap();
  let operator = args.nth(0).unwrap().chars().next().unwrap(); // turn the string into a character, unwraps are for error management, chars returns an iterator of the strings slice, next makes sure we select the first element of that slice, which returns an option that we can unwrap if the value exists
  let second = args.nth(0).unwrap();

  // converting strings to actual numbers
  let first_number: f32 = first.parse().unwrap();
  let second_number: f32 = second.parse().unwrap();

  /*
  Other way of doing it. known as Turbofish method
  let first_number = first.parse::<f32>().unwrap();
  let second_number = second.parse::<f32>().unwrap();
  */

  let result = operate(operator, first_number, second_number);

  println!(
    "{:?}",
    output(first_number, operator, second_number, result)
  );
}

/*
// do the actual computation
fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
  if operator == '+' {
    first_number + second_number // notice how there is no semi colon and no return statements: it is an implicit return
  } else if operator == '-' {
    first_number - second_number
  } else if operator == '/' {
    first_number / second_number
  } else if operator == '*' {
    first_number * second_number
  } else {
    // Null is not possible in rust so we must return something anyway, else is mandatory
    0.0 //notice the decimal, since it expects f32, appropriate notation required.
  }
}
*/


//A more idiomatic way of doing conditional logic 
fn operate(operator:char, first_number: f32, second_number: f32) -> f32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'x' | 'X' => first_number * second_number, //in the shell, asterisk need to be escaped, this is why, using single pipes, we can extend the match to 'x' and 'X' too for instance
    _ => panic!("Invalid operator used.")
  }
}




fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
  // format macro returns a string so all good
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}
