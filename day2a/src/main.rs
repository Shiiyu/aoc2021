fn main() {
  let (mut forward, mut depth) = (0, 0);
  let input: Vec<(&str, &str)> = include_str!("../input.txt")
    .lines()
    .map(|num| num.split_once(' ').unwrap_or_default())
    .collect();

  for (direction, value) in input {
    match (direction, value.parse::<i32>().unwrap_or_default()) {
      ("forward", value) => forward += value,
      ("up", value) => depth -= value,
      ("down", value) => depth += value,
      (_, _) => ()
    }
  }

  println!("Depth times horizontal: {}", forward * depth);
}
