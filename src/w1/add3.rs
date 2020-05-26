use std::env;
fn main() {
  let args: Vec<String> = env::args().collect();
  let a: u32 = args[1].parse().unwrap();
  let b: u32 = args[2].parse().unwrap();
  let c: u32 = args[3].parse().unwrap();
  let sum = a + b + c;
  println!("Sum = {}", sum);
}