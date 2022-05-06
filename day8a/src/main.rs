fn main() {
  println!(
    "Num Occurances: {}",
    include_str!("../input.txt")
      .lines()
      .flat_map(|line| line.split_once('|').unwrap().1.split_whitespace())
      .filter(|digit| matches!(digit.len(), 2 | 3 | 4 | 7))
      .count()
  );
}
