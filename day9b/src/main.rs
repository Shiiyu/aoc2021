fn find_basin(heightmap: &mut Vec<Vec<u8>>, x: usize, y: usize) -> u32 {
  let mut size = 1;

  heightmap[y][x] = b'9';

  if y != heightmap.len() - 1 && heightmap[y + 1][x] != b'9' {
    size += find_basin(heightmap, x, y + 1);
  }

  if x != heightmap[y].len() - 1 && heightmap[y][x + 1] != b'9' {
    size += find_basin(heightmap, x + 1, y);
  }

  if y != 0 && heightmap[y - 1][x] != b'9' {
    size += find_basin(heightmap, x, y - 1);
  }

  if x != 0 && heightmap[y][x - 1] != b'9' {
    size += find_basin(heightmap, x - 1, y);
  }

  size
}

fn main() {
  let mut heightmap: Vec<Vec<u8>> = include_bytes!("../input.txt").split(|&b| b == b'\n').map(|l| l.to_vec()).collect();
  let mut basins = Vec::new();

  for y in 0..heightmap.len() {
    for x in 0..heightmap[y].len() {
      if heightmap[y][x] != b'9' {
        basins.push(find_basin(&mut heightmap, x, y));
      }
    }
  }

  basins.sort_unstable();

  println!(
    "Product of 3 Largest Basins: {}",
    basins[basins.len() - 1] * basins[basins.len() - 2] * basins[basins.len() - 3]
  );
}
