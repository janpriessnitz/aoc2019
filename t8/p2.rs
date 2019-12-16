

use std::io;

fn main() {
    let mut line = String::new();
    let mut pos = 0;
    let mut pic = vec![2; 25*6];
    io::stdin().read_line(&mut line).expect("failed to read line");
    for c in line.trim().chars() {
        match c {
            '0' => if pic[pos % (25*6)] == 2 {pic[pos % (25*6)] = 0},
            '1' => if pic[pos % (25*6)] == 2 {pic[pos % (25*6)] = 1},
            '2' => {},
            _ => panic!("unknown char"),
        }
        pos += 1;
    }
    pos = 0;
    for n in pic {
        print!("{}", n);
        pos += 1;
        if pos % 25 == 0 {
            println!("");
        }
    }
}
