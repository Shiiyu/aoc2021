fn parse_digit(digit: &str, one: &str, four: &str) -> u32 {
  match digit.len() {
    2 => 1,
    3 => 7,
    4 => 4,
    7 => 8,
    5 => {
      if one.chars().all(|c| digit.contains(c)) {
        3
      } else if four.matches(|c| digit.contains(c)).count() == 2 {
        2
      } else {
        5
      }
    },
    6 => {
      if four.chars().all(|c| digit.contains(c)) {
        9
      } else if one.chars().all(|c| digit.contains(c)) {
        0
      } else {
        6
      }
    },
    _ => unreachable!(),
  }
}

fn main() {
  println!(
    "Sum Values: {}",
    include_str!("../input.txt").lines().into_iter().fold(0, |total, line| {
      let (key, numbers) = line.split_once(" | ").unwrap();
      let one = key.split_whitespace().find(|n| n.len() == 2).unwrap();
      let four = key.split_whitespace().find(|n| n.len() == 4).unwrap();

      total
        + numbers
          .split_whitespace()
          .enumerate()
          .map(|(i, n)| parse_digit(n, one, four) * u32::pow(10, 3 - i as u32))
          .sum::<u32>()
    })
  )
}
