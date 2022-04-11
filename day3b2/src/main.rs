fn life(mut values: Vec<&str>, greater: bool) -> u32 {
  let length = values[0].len();
  let mut count = values.len();
  let mut life = String::new();

  for i in 0..length {
    if count == 1 {
      life = values[0].to_owned();
      break;
    }

    let num = values.iter().filter(|n| &n[i..i + 1] == "1").count();

    if (greater && num >= count - num) || (!greater && num < count - num) {
      values.retain(|&x| x.starts_with(&(life.clone() + "1")));
      life = life.to_owned() + "1";
    } else {
      values.retain(|&x| x.starts_with(&(life.clone() + "0")));
      life = life.to_owned() + "0";
    }

    count = values.len();
  }

  u32::from_str_radix(&life, 2).unwrap_or_default()
}

fn main() {
  let input = include_str!("../input.txt");
  let oxygen = life(input.lines().collect(), true);
  let co2 = life(input.lines().collect(), false);

  println!("Life Support Rating: {}", oxygen * co2);
}
