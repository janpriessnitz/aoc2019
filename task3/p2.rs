
use std::io;
use std::collections::HashMap;

fn parse_instr(instr: &str) -> (i32, i32, i32) {
  let diff : i32 = instr[1..].parse().expect("not a number");
  match &instr[0..1] {
    "U" => (0, -1, diff),
    "D" => (0, 1, diff),
    "L" => (-1, 0, diff),
    "R" => (1, 0, diff),
    _ => (0, 0, 0),
  }
}

fn main() {
  let mut occupied = HashMap::new();
  let mut result = 1000000;
  let mut steps = 0;
  let mut x = 0;
  let mut y = 0;
  let mut line1 = String::new();
  io::stdin().read_line(&mut line1).expect("No line");
  for instr in line1.trim().split(",") {
    let (dx, dy, diff) = parse_instr(instr);
    for _ in 0..diff {
      x += dx;
      y += dy;
      steps += 1;
      occupied.insert((x, y), steps);
    }
  }

  x = 0;
  y = 0;
  steps = 0;
  let mut line2 = String::new();
  io::stdin().read_line(&mut line2).expect("No line");
  for instr in line2.trim().split(",") {
    let (dx, dy, diff) = parse_instr(instr);
    for _ in 0..diff {
      x += dx;
      y += dy;
      steps += 1;
      result = match occupied.get(&(x, y)) {
        None => result,
        Some(distance) => if distance + steps < result {
          distance + steps
        } else {
          result
        },
      }
    }
  }
  println!("result {}", result);
}