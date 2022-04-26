const LENGTH: usize = 12;

fn solve(mut vec: Vec<u32>, greater: bool) -> u32 {
  for i in (0..LENGTH).rev() {
    if vec.len() == 1 {
      break;
    }

    let which = ((vec.iter().filter(|n| (*n & (1 << i)) >> i == 1).count() >= (vec.len() + 1) / 2) == greater) as u32;

    vec.retain(|n| ((n & (1 << i)) >> i) == which);
  }

  vec[0]
}

fn main() {
  let oxy: Vec<u32> = include_str!("../input.txt").lines().map(|l| u32::from_str_radix(l, 2).unwrap()).collect();
  let co2 = oxy.clone();

  println!("Life Support Rating: {}", solve(oxy, true) * solve(co2, false));
}
