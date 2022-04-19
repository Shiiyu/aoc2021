const LENGTH: usize = 12;
const COUNT: usize = 1000;

fn main() {
  let input: Vec<&str> = include_str!("../input.txt").lines().collect();
  let mut string = String::new();

  for i in 0..LENGTH {
    let mut num = 0;

    for entry in &input {
      if &entry[i..i + 1] == "1" {
        num += 1;
      }
    }

    if num >= COUNT - num {
      string.push('1');
    } else {
      string.push('0');
    }
  }

  let gamma = u32::from_str_radix(&string, 2).unwrap();
  let epsilon = (1 << LENGTH) - 1 - gamma;

  println!("Power Usage: {}", gamma * epsilon);
}
