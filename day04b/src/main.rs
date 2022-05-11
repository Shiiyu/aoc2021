const SIZE: usize = 25;
const ROW: u32 = 0b11111;
const COL: u32 = 0b100001000010000100001;

trait IntoArray: IntoIterator {
  fn into_array<const N: usize>(self) -> [Self::Item; N];
}

impl<I: IntoIterator> IntoArray for I {
  fn into_array<const N: usize>(self) -> [Self::Item; N] {
    let mut iter = self.into_iter();

    [(); N].map(|_| iter.next().unwrap())
  }
}

struct Board {
  tiles: [u8; SIZE],
  drawn: u32
}

impl Board {
  fn new<I: Iterator<Item = u8>>(iter: I) -> Self {
    Self { tiles: iter.into_array(), drawn: 0 }
  }

  fn draw(&mut self, roll: u8) {
    self.tiles.iter().enumerate().filter(|(_, &t)| t == roll).for_each(|(i, _)| self.drawn |= 1 << i);
  }

  fn is_solved(&self) -> bool {
    (0..5).any(|i| self.drawn >> (i * 5) & ROW == ROW || self.drawn >> i & COL == COL)
  }

  fn value(&self, roll: u8) -> u32 {
    self.tiles.into_iter().enumerate().map(|(i, t)| (self.drawn >> i & 1 ^ 1) * t as u32).sum::<u32>() * roll as u32
  }
}

fn input() -> (Vec<u8>, Vec<Board>) {
  let sections = include_str!("../input.txt").split_once("\n\n").unwrap();

  (
    sections.0.split(',').map(|n| n.parse().unwrap()).collect(),
    sections.1.split("\n\n").map(|b| Board::new(b.split_whitespace().map(|n| n.parse().unwrap()))).collect()
  )
}

fn main() {
  let (rolls, mut boards) = input();

  for roll in rolls.into_iter() {
    let mut i: usize = 0;

    while i < boards.len() {
      boards[i].draw(roll);

      if boards[i].is_solved() {
        if boards.len() == 1 {
          return println!("Losing Board: {}", boards[i].value(roll));
        } else {
          boards.remove(i);

          if i == 0 {
            continue;
          } else {
            i -= 1;
          }
        }
      }

      i += 1;
    }
  }
}
