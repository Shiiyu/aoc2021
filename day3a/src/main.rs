fn main() {
  let input: Vec<&str> = include_str!("../input.txt").lines().collect();
  let length = input[0].len();
  let count = input.len();
  let mut string = String::new();

  for i in 0..length {
    let mut num = 0;

    for entry in &input {
      if &entry[i..i + 1] == "1" {
        num += 1;
      }
    }

    if num >= count - num {
      string.push('1');
    } else {
      string.push('0');
    }
  }

  let gamma = u32::from_str_radix(&string, 2).unwrap_or_default();
  let epsilon = (1 << length) - 1 - gamma;

  println!("Power Usage: {}", gamma * epsilon);
}
