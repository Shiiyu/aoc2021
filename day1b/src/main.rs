fn main() {
  println!(
    "Num times sum measurements increased: {}",
    include_str!("../input.txt")
      .lines()
      .map(|num| num.parse().unwrap())
      .collect::<Vec<u16>>()
      .windows(4)
      .filter(|d| d[0] < d[3])
      .count()
  );
}
