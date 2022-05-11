const LENGTH: usize = 12;
const THRESHOLD: u32 = 500;

fn main() {
  let input: Vec<u32> = include_str!("../input.txt").lines().map(|l| u32::from_str_radix(l, 2).unwrap()).collect();
  let mut num_vec = vec![0; LENGTH];

  input.iter().for_each(|n| {
    for i in 0..LENGTH {
      num_vec[i] += (n & (1 << i)) >> i;
    }
  });

  let gamma: u32 = num_vec.into_iter().enumerate().map(|(i, n)| ((n >= THRESHOLD) as u32) << i).sum();
  let epsilon = (1 << LENGTH) - 1 - gamma;

  println!("Power Usage: {}", gamma * epsilon);
}
