fn main() {
  let mut count = 0;
  let nums: Vec<u16> =
    include_str!("../input.txt").lines().map(|num| num.parse().unwrap_or_default()).collect();
  let max = nums.len() - 2;

  for i in 1..max {
    let sum_one = nums[i - 1] + nums[i] + nums[i + 1];
    let sum_two = nums[i] + nums[i + 1] + nums[i + 2];

    if sum_two > sum_one {
      count += 1;
    }
  }

  println!("Num times sum measurements increased: {}", count);
}
