struct Board {
  nums: [[usize; 5]; 5],
  drawn: [[bool; 5]; 5],
  moves: usize,
  move_win: usize
}

impl Board {
  fn bingo(&mut self, nums: &[usize]) {
    for (i, num) in nums.iter().enumerate() {
      for x in 0..5 {
        for y in 0..5 {
          if num == &self.nums[x][y] {
            self.drawn[x][y] = true;

            if self.check_if_win() {
              self.move_win = *num;
              self.moves = i;

              return;
            }
          }
        }
      }
    }
  }

  fn calc_score(&self) -> usize {
    let Board { nums, drawn, moves: _, move_win } = self;
    let mut sum = 0usize;

    for x in 0..5 {
      for y in 0..5 {
        if !drawn[x][y] {
          sum += nums[x][y];
        }
      }
    }

    sum * move_win
  }

  fn check_if_win(&self) -> bool {
    let drawn = self.drawn;

    for i in 0..5 {
      if drawn[i][0] && drawn[i][1] && drawn[i][2] && drawn[i][3] && drawn[i][4] {
        return true;
      }
      if drawn[0][i] && drawn[1][i] && drawn[2][i] && drawn[3][i] && drawn[4][i] {
        return true;
      }
    }

    false
  }

  fn moves(&self) -> usize {
    self.moves
  }

  fn new(input: &str) -> Self {
    let mut nums = [[0usize; 5]; 5];
    let mut nums_str = input.split_whitespace();
    let drawn = [[false; 5]; 5];
    let moves = 0usize;
    let move_win = 0usize;

    for x in &mut nums {
      for y in x {
        let num_str = nums_str.next().unwrap_or_default();

        *y = num_str.parse().unwrap_or_default();
      }
    }

    Self { nums, drawn, moves, move_win }
  }
}

fn main() {
  let (nums_str, boards) = include_str!("../input.txt").split_once("\r\n\r\n").unwrap_or_default();
  let nums: Vec<usize> = nums_str.split(',').map(|n| n.parse::<usize>().unwrap_or_default()).collect();
  let (mut moves, mut score) = (nums.len(), 0usize);

  boards.split("\r\n\r\n").for_each(|s| {
    let mut board = Board::new(s);

    board.bingo(&nums);

    if board.moves() < moves {
      moves = board.moves();
      score = board.calc_score();
    }
  });

  println!("Board Score: {score}");
}
