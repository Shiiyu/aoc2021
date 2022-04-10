use std::fs;

fn main() {
  let (mut forward, mut depth, mut aim) = (0, 0, 0);
  let input = fs::read_to_string("./input.txt").unwrap_or_default();
  let input_vec: Vec<(&str, &str)> = input.lines()
    .map(|num| num.split_once(' ').unwrap_or_default())
    .collect();

  for (direction, value) in input_vec {
    match (direction, value.parse::<i32>().unwrap_or_default()) {
      ("forward", value) => {
        forward += value;
        depth += aim * value;
      },
      ("up", value) => aim -= value,
      ("down", value) => aim += value,
      (_, _) => ()
    }
  }

  println!("Depth times horizontal: {}", forward * depth);
}