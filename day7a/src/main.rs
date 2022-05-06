fn main() {
  let mut crabs: Vec<u16> = include_str!("../input.txt").split(',').map(|n| n.parse().unwrap()).collect();
  crabs.sort_unstable();
  let align = crabs[crabs.len() / 2];

  println!("Fuel Usage: {}", crabs.iter().fold(0, |fuel, crab| fuel + crab.abs_diff(align) as u32));
}
