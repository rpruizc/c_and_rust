const MILES: f32 = 26.0;
const YARDS: f32 = 385.0;

fn main() {
  let mut kilometers: f32 = 0.0;

  kilometers = 1.609 * (MILES + YARDS/1760.0);

  println!("A marathon is {} kilometers", kilometers);
}