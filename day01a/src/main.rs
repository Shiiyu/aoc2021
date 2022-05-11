fn main() {
  println!(
    "Num of larger measurements: {}",
    include_str!("../input.txt")
      .lines()
      .map(|num| num.parse().unwrap())
      .collect::<Vec<u16>>()
      .windows(2)
      .filter(|d| d[0] < d[1])
      .count()
  );
}
