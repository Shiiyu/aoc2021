use std::fs;

fn main() {
  let mut count = 0;
  let numstr = fs::read_to_string("./input.txt").unwrap_or_default();
  let nums: Vec<u16> = numstr.lines().map(|num| num.parse().unwrap_or_default()).collect();

  for i in 1..nums.len() {
    if nums[i] > nums[i - 1] {
      count += 1;
    }
  }

  println!("Num of larger measurements: {}", count);
}
