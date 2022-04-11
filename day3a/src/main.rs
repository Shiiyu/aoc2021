use std::fs;

fn main() {
  let input = fs::read_to_string("./input.txt").unwrap_or_default();
  let input_lines: Vec<&str> = input.lines().collect();
  let length = input_lines[0].len();
  let count = input_lines.len();
  let mut string = String::new();

  for i in 0..length {
    let mut num = 0;

    for entry in &input_lines {
      if &entry[i..i + 1] == "1" {
        num += 1;
      }
    }

    if num % (count / 2) < num {
      string = string.to_owned() + "1";
    } else {
      string = string.to_owned() + "0";
    }
  }

  let gamma = u32::from_str_radix(&string, 2).unwrap_or_default();
  let epsilon = (2u32.pow(length.try_into().unwrap_or_default()) - 1) - gamma;

  println!("Power Usage: {}", gamma * epsilon);
}
