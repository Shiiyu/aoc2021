use std::fs;

fn main() {
  let input = fs::read_to_string("./input.txt").unwrap_or_default();
  let mut fish: Vec<u8> = input.split(',').map(|x| x.parse().unwrap_or_default()).collect();

  for _ in 0..80 {
    for f in 0..fish.len() {
      match fish[f] {
        0 => {
          fish.push(8);
          fish[f] = 6;
        },
        _ => fish[f] -= 1
      }
    }
  }

  println!("Fish after 80 days: {}", fish.len());
}
