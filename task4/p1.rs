

fn is_correct(inputt: i32) -> bool {
  let mut last_digit = 10;
  let mut last_last_digit = 11;
  let mut double_digits = 0;
  let mut input = inputt;
  for _ in 0..6 {
    let digit = input % 10;
    if digit > last_digit {
      return false;
    }
    if double_digits == digit && last_last_digit == digit {
      double_digits = 0;
    }
    if double_digits == 0 && digit == last_digit && digit != last_last_digit {
      double_digits = digit;
    }
    last_last_digit = last_digit;
    last_digit = digit;
    input /= 10;
  }
  if double_digits != 0 {
    println!("{}", inputt);
    return true;
  } else {
    return false;
  }
}

fn main() {
  let min = 382345;
  let max = 843167;
  let mut result = 0;
  for i in min..(max+1) {
    result += match is_correct(i) {
      false => 0,
      true => 1,
    }
  }
  println!("result {}", result);
}