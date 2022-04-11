use std::fs;

pub fn main() {
  let input = fs::read_to_string("./input.txt").unwrap_or_default();
  let mut fish = input.split(',').fold([0; 9], |mut f, num| {
    f[num.parse::<usize>().unwrap_or_default()] += 1;
    f
  });

  for i in 1..256 {
    fish[(i + 7) % 9] += fish[i % 9];
  }

  let end_fish: usize = fish.iter().sum();

  println!("End Fish: {end_fish}");
}
