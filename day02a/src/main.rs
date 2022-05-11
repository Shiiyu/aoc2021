fn main() {
  let (forward, depth) = include_str!("../input.txt").lines().map(|num| num.split_once(' ').unwrap()).fold(
    (0, 0),
    |(f, d), (c, v)| match (c, v.parse::<i32>().unwrap()) {
      ("forward", v) => (f + v, d),
      ("up", v) => (f, d - v),
      ("down", v) => (f, d + v),
      _ => unreachable!()
    }
  );

  println!("Depth times horizontal: {}", forward * depth);
}
