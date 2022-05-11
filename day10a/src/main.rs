fn main() {
  println!(
    "Syntax Error Score: {}",
    include_str!("../input.txt").lines().fold(0, |total, line| {
      let mut openings = Vec::new();

      for bracket in line.chars() {
        if matches!(bracket, '(' | '[' | '{' | '<') {
          openings.push(bracket);
        } else if bracket == ')' {
          if openings.pop() != Some('(') {
            return total + 3;
          }
        } else if bracket == ']' {
          if openings.pop() != Some('[') {
            return total + 57;
          }
        } else if bracket == '}' {
          if openings.pop() != Some('{') {
            return total + 1197;
          }
        } else if bracket == '>' {
          if openings.pop() != Some('<') {
            return total + 25137;
          }
        }
      }

      total
    })
  );
}
