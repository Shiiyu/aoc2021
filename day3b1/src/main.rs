use std::fs;

fn main() {
  // This huge mess could easily be avoided by making a struct, but who needs good
  // code when it still works :) On that note i'm rewriting it because i'm
  // really unhappy with how this looks
  let input = fs::read_to_string("./input.txt").unwrap_or_default();
  let mut input_lines_life: Vec<&str> = input.lines().collect();
  let mut input_lines_co2 = input_lines_life.clone();
  let length = input_lines_life[0].len();
  let mut count_life = input_lines_life.len();
  let mut count_co2 = input_lines_co2.len();
  let mut string_life = String::new();
  let mut string_co2 = String::new();

  for i in 0..length {
    if count_life == 1 {
      string_life = input_lines_life[0].to_owned();
      break;
    }

    let num = input_lines_life.iter().filter(|n| &n[i..i + 1] == "1").count();

    if num >= count_life - num {
      input_lines_life.retain(|&x| x[0..i + 1] == string_life.clone() + "1");
      string_life = string_life.to_owned() + "1";
    } else {
      input_lines_life.retain(|&x| x[0..i + 1] == string_life.clone() + "0");
      string_life = string_life.to_owned() + "0";
    }

    count_life = input_lines_life.len();
  }

  for i in 0..length {
    if count_co2 == 1 {
      string_co2 = input_lines_co2[0].to_owned();
      break;
    }

    let num = input_lines_co2.iter().filter(|n| &n[i..i + 1] == "1").count();

    if num < count_co2 - num {
      input_lines_co2.retain(|&x| x[0..i + 1] == string_co2.clone() + "1");
      string_co2 = string_co2.to_owned() + "1";
    } else {
      input_lines_co2.retain(|&x| x[0..i + 1] == string_co2.clone() + "0");
      string_co2 = string_co2.to_owned() + "0";
    }

    count_co2 = input_lines_co2.len();
  }

  let life = u32::from_str_radix(&string_life, 2).unwrap_or_default();
  let co2 = u32::from_str_radix(&string_co2, 2).unwrap_or_default();

  println!("Life Support Rating: {}", life * co2);
}
