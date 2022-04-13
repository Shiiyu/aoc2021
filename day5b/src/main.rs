use std::thread;

fn puzzle() {
  let input: Vec<&str> = include_str!("../input.txt").lines().collect();
  let mut grid = [[0usize; 1000]; 1000];
  let mut count = 0usize;

  for i in input {
    let mut split = i.split(" -> ");
    let first = split.next().unwrap_or_default();
    let second = split.next().unwrap_or_default();
    let (x1_str, y1_str) = first.split_once(',').unwrap_or_default();
    let (x2_str, y2_str) = second.split_once(',').unwrap_or_default();
    let x1 = x1_str.parse::<usize>().unwrap_or_default();
    let x2 = x2_str.parse::<usize>().unwrap_or_default();
    let mut y1 = y1_str.parse::<usize>().unwrap_or_default();
    let mut y2 = y2_str.parse::<usize>().unwrap_or_default();

    if x1 == x2 {
      if y1 < y2 {
        for i in y1..=y2 {
          grid[x1][i] += 1;
        }
      } else {
        for i in y2..=y1 {
          grid[x1][i] += 1;
        }
      }
    } else if y1 == y2 {
      if x1 < x2 {
        for i in x1..=x2 {
          grid[i][y1] += 1;
        }
      } else {
        for i in x2..=x1 {
          grid[i][y1] += 1;
        }
      }
    } else if x1 > x2 {
      if y1 > y2 {
        for i in x2..=x1 {
          grid[i][y2] += 1;
          y2 += 1;
        }
      } else {
        for i in x2..=x1 {
          grid[i][y2] += 1;
          y2 -= 1;
        }
      }
    } else {
      if y1 > y2 {
        for i in x1..=x2 {
          grid[i][y1] += 1;
          y1 -= 1;
        }
      } else {
        for i in x1..=x2 {
          grid[i][y1] += 1;
          y1 += 1;
        }
      }
    }
  }

  for x in 0..1000 {
    for y in 0..1000 {
      if grid[x][y] >= 2 {
        count += 1;
      }
    }
  }

  println!("{count}");
}

fn main() {
  let child = thread::Builder::new().stack_size(8 * 1024 * 1024).spawn(puzzle).unwrap();

  child.join().unwrap();
}
