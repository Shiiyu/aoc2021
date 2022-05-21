#![feature(array_windows)]

fn main() {
  println!(
    "Num of larger measurements: {}",
    include_str!("../input.txt")
      .lines()
      .map(|num| num.parse().unwrap())
      .collect::<Vec<u16>>()
      .array_windows()
      .filter(|[a, b]| a < b)
      .count()
  );
}
