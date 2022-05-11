fn main() {
  let (forward, depth, _) = include_str!("../input.txt").lines().map(|num| num.split_once(' ').unwrap()).fold(
    (0, 0, 0),
    |(f, d, a), (c, v)| match (c, v.parse::<i32>().unwrap()) {
      ("forward", v) => (f + v, d + a * v, a),
      ("down", v) => (f, d, a + v),
      ("up", v) => (f, d, a - v),
      _ => unreachable!()
    }
  );

  println!("Depth times horizontal: {}", forward * depth);
}
