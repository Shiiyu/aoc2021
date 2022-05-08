fn main() {
  let heightmap: Vec<Vec<u8>> =
    include_str!("../input.txt").lines().map(|l| l.chars().map(|n| n as u8 - '0' as u8).collect()).collect();
  let mut sum = 0;

  for y in 0..heightmap.len() {
    for x in 0..heightmap[y].len() {
      if (x == 0 || heightmap[y][x - 1] > heightmap[y][x])
        && (y == 0 || heightmap[y - 1][x] > heightmap[y][x])
        && (x == heightmap[y].len() - 1 || heightmap[y][x + 1] > heightmap[y][x])
        && (y == heightmap.len() - 1 || heightmap[y + 1][x] > heightmap[y][x])
      {
        sum += (heightmap[y][x] + 1) as u16;
      }
    }
  }

  println!("Sum of Risk Levels: {sum}");
}
