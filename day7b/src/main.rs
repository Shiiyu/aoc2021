fn main() {
  let crabs: Vec<usize> = include_str!("../input.txt").split(',').map(|n| n.parse().unwrap_or_default()).collect();
  let min = crabs.iter().min().unwrap().clone();
  let max = crabs.iter().max().unwrap().clone();
  let mut min_fuel = usize::MAX;

  for i in min..=max {
    let mut current_fuel = 0;

    crabs.iter().for_each(|&p| {
      let math = (i as isize - p as isize).abs();
      current_fuel += (math * (math + 1)) / 2
    });

    let current = current_fuel.try_into().unwrap_or_default();

    if current < min_fuel {
      min_fuel = current;
    }
  }

  println!("{min_fuel:?}");
}
