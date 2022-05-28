const OFFSET: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
const SIZE: usize = 10;

fn flash(octopi: &mut Vec<Vec<u8>>, x: usize, y: usize) -> usize {
  octopi[y][x] = b'0';
  OFFSET.iter().map(|(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize)).fold(1, |flashes, (x, y)| {
    match octopi.get_mut(y).and_then(|l| l.get_mut(x)) {
      Some(octopus) if *octopus > b'0' => {
        *octopus += 1;
        flashes + (*octopus > b'9').then(|| flash(octopi, x, y)).unwrap_or(0)
      },
      _ => flashes
    }
  })
}

fn main() {
  let mut octopi: Vec<Vec<u8>> = include_bytes!("../input.txt").split(|&b| b == b'\n').map(|l| l.to_vec()).collect();

  println!(
    "All Flash On Step: {}",
    (1..)
      .find(|_| {
        octopi.iter_mut().for_each(|l| l.iter_mut().for_each(|c| *c += 1));

        let flashes = (0..SIZE)
          .flat_map(|y| (0..SIZE).map(move |x| (x, y)))
          .fold(0, |flashes, (x, y)| flashes + (octopi[y][x] > b'9').then(|| flash(&mut octopi, x, y)).unwrap_or(0));

        flashes == SIZE * SIZE
      })
      .unwrap()
  )
}
