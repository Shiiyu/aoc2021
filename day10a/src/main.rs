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
  println!(
    "Syntax Error Score: {}",
    include_str!("../input.txt").lines().fold(0, |total, line| {
      let mut openings = Vec::new();

      for bracket in line.chars() {
        if matches!(bracket, '(' | '[' | '{' | '<') {
          openings.push(bracket);
        } else if equals(openings[openings.len() - 1], bracket) {
          openings.pop();
        } else {
          return match bracket {
            ')' => total + 3,
            ']' => total + 57,
            '}' => total + 1197,
            '>' => total + 25137,
            _ => unreachable!()
          };
        }
      }

      total
    })
  );
}
