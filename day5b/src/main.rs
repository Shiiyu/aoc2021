use std::{
  cmp::{max, min},
  collections::HashSet
};

#[derive(Clone, Copy)]
struct Line {
  x: u16,
  y: u16,
  dx: u16,
  dy: u16
}

impl Line {
  fn new(s: &str) -> Self {
    let (start, end) = s.split_once(" -> ").unwrap();

    let (x, y) = start.split_once(',').map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap())).unwrap();
    let (dx, dy) = end.split_once(',').map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap())).unwrap();

    if x <= dx {
      Self { x, y, dx, dy }
    } else {
      Self { x: dx, y: dy, dx: x, dy: y }
    }
  }

  fn sorted_y(&self) -> (u16, u16) {
    if self.y < self.dy {
      (self.y, self.dy)
    } else {
      (self.dy, self.y)
    }
  }

  // (a, b) from ax + by + c = 0
  fn ab(&self) -> (f32, f32) {
    if self.y == self.dy {
      (0.0, 1.0)
    } else if self.x == self.dx {
      (1.0, 0.0)
    } else if self.y < self.dy {
      (-1.0, 1.0)
    } else {
      (1.0, 1.0)
    }
  }

  // c from ax + by + c = 0
  fn c(&self) -> f32 {
    if self.y == self.dy {
      -(self.y as f32)
    } else if self.x == self.dx {
      -(self.x as f32)
    } else if self.y < self.dy {
      self.x as f32 - self.y as f32
    } else {
      -(self.x as f32) - self.y as f32
    }
  }

  fn can_layer(&self, other: &Self) -> bool {
    if self.y == self.dy {
      other.y == other.dy
    } else if other.y == other.dy {
      self.y == self.dy
    } else if self.x == self.dx {
      other.x == other.dx
    } else if other.x == other.dx {
      self.x == self.dx
    } else {
      (self.y < self.dy && other.y < other.dy) || (self.y > self.dy && other.y > other.dy)
    }
  }

  fn contains(&self, x: u16, y: u16) -> bool {
    let ys = self.sorted_y();

    self.x <= x && self.dx >= x && ys.0 <= y && ys.1 >= y
  }
}

fn layer(first: &Line, second: &Line) -> Vec<(u16, u16)> {
  let mut list = Vec::new();

  if first.x == first.dx {
    let f = first.sorted_y();
    let s = second.sorted_y();

    for y in max(f.0, s.0)..=min(f.1, s.1) {
      list.push((first.x, y));
    }
  } else if first.y == first.dy {
    for x in max(first.x, second.x)..=min(first.dx, second.dx) {
      list.push((x, first.y));
    }
  } else if first.dx >= second.x && second.dx >= first.x {
    let start = if first.x < second.x { (second.x, second.y) } else { (first.x, first.y) };

    if first.y < first.dy {
      for i in 0..=(min(first.dx, second.dx) - start.0) {
        list.push((start.0 + i, start.1 + i));
      }
    } else {
      for i in 0..=(min(first.dx, second.dx) - start.0) {
        list.push((start.0 + i, start.1 - i));
      }
    }
  }

  list
}

fn intersection(first: &Line, second: &Line) -> Option<(u16, u16)> {
  let (a1, b1) = first.ab();
  let (a2, b2) = second.ab();
  let (c1, c2) = (first.c(), second.c());

  let d = (a1 * b2) - (a2 * b1);
  let x = ((b1 * c2) - (b2 * c1)) / d;
  let y = ((c1 * a2) - (c2 * a1)) / d;

  if x % 1.0 != 0.0 || y % 1.0 != 0.0 {
    return None;
  }

  let (x, y) = (x as u16, y as u16);

  if first.contains(x, y) && second.contains(x, y) {
    Some((x, y))
  } else {
    None
  }
}
fn main() {
  let lines: Vec<Line> = include_str!("../input.txt").lines().map(|n| Line::new(n)).collect();
  let mut overlaps = HashSet::new();

  for (i, first) in lines.iter().enumerate() {
    for second in lines.iter().skip(i + 1) {
      if first.can_layer(second) {
        if first.c() == second.c() {
          layer(first, second).into_iter().for_each(|point| {
            overlaps.insert(point);
          });
        }
      } else {
        intersection(first, second).map(|point| overlaps.insert(point));
      }
    }
  }

  println!("Overlaps: {}", overlaps.len());
}
