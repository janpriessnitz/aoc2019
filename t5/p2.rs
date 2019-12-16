

use std::io;

fn get_val(mode: isize, parameter: isize, tape: &Vec<isize>) -> isize {
    match mode {
        0 => return tape[parameter as usize],
        1 => return parameter,
        x => {println!("unknown mode {}", x); return 0},
    }
}

fn main() {
	let mut line = String::new();
	io::stdin().read_line(&mut line).expect("Failed to read line");
	let split = line.split(",").map(|x| x.trim().parse::<isize>().expect("failed to parse"));
	let mut tape: Vec<isize> = split.collect();
	let mut pos = 0;
	loop {
        let mode1 = (tape[pos] / 100) % 10;
        let mode2 = (tape[pos] / 1000) % 10;
        let mode3 = (tape[pos] / 10000) % 10;
		match tape[pos] % 100 {
            1 => {let dest = tape[pos+3] as usize;
                  tape[dest] = get_val(mode1, tape[pos+1], &tape) + get_val(mode2, tape[pos+2], &tape);
                  pos += 4; },
			2 => {let dest = tape[pos+3] as usize;
                  tape[dest] = get_val(mode1, tape[pos+1], &tape)*get_val(mode2, tape[pos+2], &tape);
                  pos += 4; },
            3 => {let dest = tape[pos+1] as usize;
                  tape[dest] = 5;
                  pos += 2},
            4 => {println!("output: {}", get_val(mode1, tape[pos+1], &tape));
                  pos += 2},
            5 => {match get_val(mode1, tape[pos+1], &tape) {
                    0 => {pos += 3},
                    _ => {pos = get_val(mode2, tape[pos+2], &tape) as usize},
                 }},
            6 => {match get_val(mode1, tape[pos+1], &tape) {
                    0 => {pos = get_val(mode2, tape[pos+2], &tape) as usize},
                    _ => {pos += 3},
                 }},
            7 => {let store_val;
                  if get_val(mode1, tape[pos+1], &tape) < get_val(mode2, tape[pos+2], &tape) {
                     store_val = 1;
                  } else {
                     store_val = 0;
                  }
                  let dest = tape[pos+3] as usize;
                  tape[dest] = store_val;
                  pos += 4},
            8 => {let store_val;
                  if get_val(mode1, tape[pos+1], &tape) == get_val(mode2, tape[pos+2], &tape) {
                     store_val = 1;
                  } else {
                     store_val = 0;
                  }
                  let dest = tape[pos+3] as usize;
                  tape[dest] = store_val;
                  pos += 4},
			99 => break,
			x => {println!("unknown op {}", x); break},
		}
	}
}
