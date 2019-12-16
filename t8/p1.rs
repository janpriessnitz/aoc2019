

use std::io;

fn main() {
    let mut line = String::new();
    let mut min0 = 1000;
    let mut min_result = 0;
    let mut counts = (1000, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    let mut pos = 0;
    io::stdin().read_line(&mut line).expect("failed to read line");
    for c in line.trim().chars() {
        match c {
            '0' => counts.0 += 1,
            '1' => counts.1 += 1,
            '2' => counts.2 += 1,
            '3' => counts.3 += 1,
            '4' => counts.4 += 1,
            '5' => counts.5 += 1,
            '6' => counts.6 += 1,
            '7' => counts.7 += 1,
            '8' => counts.8 += 1,
            '9' => counts.9 += 1,
            _ => panic!("unknown char"),
        }
        pos += 1;
        if (pos % (25*6)) == 0 {
            println!("{} {}", counts.0, pos);
            if min0 > counts.0 {
                min0 = counts.0;
                min_result = counts.1 * counts.2;
            }
            counts = (0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
        }
    }
    println!("{} {}", min0, min_result);
}
