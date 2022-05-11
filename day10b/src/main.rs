fn equals(a: char, b: char) -> bool {
  match a {
    '(' => b == ')',
    '[' => b == ']',
    '{' => b == '}',
    '<' => b == '>',
    _ => unreachable!()
  }
}

fn main() {
  let mut incompletes: Vec<u64> = include_str!("../input.txt")
    .lines()
    .map(|line| {
      let mut openings = Vec::new();

      for bracket in line.chars() {
        if matches!(bracket, '(' | '[' | '{' | '<') {
          openings.push(bracket);
        } else if equals(openings[openings.len() - 1], bracket) {
          openings.pop();
        } else {
          return 0;
        }
      }

      openings.iter().rev().fold(0, |total, bracket| {
        (total * 5)
          + match bracket {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => unreachable!()
          }
      })
    })
    .filter(|&n| n != 0)
    .collect();

  incompletes.sort_unstable();

  println!("Middle Auto-complete Score: {}", incompletes[incompletes.len() / 2]);
}
