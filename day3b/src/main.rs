const LENGTH: usize = 12;

fn count(vec: &Vec<u32>, i: usize) -> (u32, u32) {
  let mut result: (u32, u32) = (0, 0);

  for num in vec {
    result.0 += (num & (1 << i)) >> i;
    result.1 += 1
  }

  result
}

fn solve(mut vec: Vec<u32>, greater: bool) -> u32 {
  for i in (0..LENGTH).rev() {
    let amount = count(&vec, i);

    if amount.1 > 1 {
      let which = ((amount.0 >= (amount.1 + 1) / 2) == greater) as u32;

      vec.retain(|n| ((n & (1 << i)) >> i) == which);
    } else {
      break;
    }
  }

  vec[0]
}

fn main() {
  let oxy: Vec<u32> =
    include_str!("../input.txt").lines().map(|l| u32::from_str_radix(l, 2).unwrap()).collect();
  let co2 = oxy.clone();

  println!("Life Support Rating: {}", solve(oxy, true) * solve(co2, false));
}
