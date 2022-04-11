fn main() {
  let mut count = 0;
  let nums: Vec<u16> =
    include_str!("../input.txt").lines().map(|num| num.parse().unwrap_or_default()).collect();

  for i in 1..nums.len() {
    if nums[i] > nums[i - 1] {
      count += 1;
    }
  }

  println!("Num of larger measurements: {}", count);
}
