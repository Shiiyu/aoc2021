fn main() {
  let mut fish: Vec<u8> =
    include_str!("../input.txt").split(',').map(|x| x.parse().unwrap_or_default()).collect();

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
