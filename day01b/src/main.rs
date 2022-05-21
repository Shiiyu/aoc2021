#![feature(array_windows)]

fn main() {
  println!(
    "Num times sum measurements increased: {}",
    include_str!("../input.txt")
      .lines()
      .map(|num| num.parse().unwrap())
      .collect::<Vec<u16>>()
      .array_windows()
      .filter(|[a, _, _, d]| a < d)
      .count()
  );
}
