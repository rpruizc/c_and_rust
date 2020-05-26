use std::env;

const PI: f32 = 3.14159;

fn main() {
  let args: Vec<String> = env::args().collect();
  let radius: f32 = args[1].parse().unwrap();

  let area = PI * radius * radius;

  println!("Radius of {} meters; area is {} sq.meters", radius, area);
}