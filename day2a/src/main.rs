use std::fs;

fn main() {
  let (mut forward, mut depth) = (0, 0);
  let input = fs::read_to_string("./input.txt").unwrap_or_default();
  let input_vec: Vec<(&str, &str)> = input.lines()
    .map(|num| num.split_once(' ').unwrap_or_default())
    .collect();

  for i in 0..input_vec.len() {
    let direction = input_vec[i].0;
    let value = input_vec[i].1;

    match (direction, value.parse::<i32>().unwrap_or_default()) {
      ("forward", value) => forward += value,
      ("up", value) => depth -= value,
      ("down", value) => depth += value,
      (_, _) => ()
    }
  }

  println!("Depth times horizontal: {}", forward * depth);
}
