fn main() {
  let crabs: Vec<u32> = include_str!("../input.txt").split(',').map(|n| n.parse().unwrap()).collect();

  println!(
    "Fuel Usage: {}",
    (crabs.iter().sum::<u32>() / crabs.len() as u32..)
      .take(2)
      .map(|pos| {
        crabs.iter().fold(0, |fuel, crab| {
          let diff = crab.abs_diff(pos);

          fuel + diff * (diff + 1) / 2
        })
      })
      .min()
      .unwrap()
  );
}
