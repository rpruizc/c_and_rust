/* 
  Converts fahrenheit to celsius
  C = (F -32)/1.8
*/

use std::env;

fn main() {
  let fahrenheit:f32 = 0.0;
  let mut celsius:f32 = 0.0;

  let args: Vec<String> = env::args().collect();
  let fahrenheit: f32 = args[1].parse().unwrap();

  celsius = (fahrenheit - 32.0)/1.8;

  println!("{} fahrenheit is {} celsius.", fahrenheit, celsius);

}