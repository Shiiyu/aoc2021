use std::fs;

fn main() {
  let input = fs::read_to_string("./input.txt").unwrap_or_default();
  let crabs: Vec<usize> = input.split(',').map(|n| n.parse().unwrap_or_default()).collect();
  let min = crabs.iter().min().unwrap().clone();
  let max = crabs.iter().max().unwrap().clone();
  let mut min_fuel = (usize::MAX, 0);

  for i in min..=max {
    let mut current_fuel = 0;

    crabs.iter().for_each(|&p| current_fuel += (i as isize - p as isize).abs());

    let current = current_fuel.try_into().unwrap_or_default();

    if current < min_fuel.0 {
      min_fuel.0 = current;
      min_fuel.1 = i;
    }
  }

  println!("{min_fuel:?}");
}
