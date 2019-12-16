

use std::io;
use std::cmp;

fn get_val(mode: isize, parameter: isize, tape: &Vec<isize>) -> isize {
    match mode {
        0 => return tape[parameter as usize],
        1 => return parameter,
        x => {println!("unknown mode {}", x); return 0},
    }
}

fn compute(phase: isize, inp: isize, source: &Vec<isize>) -> isize {
    let mut tape = source.clone();
    let mut result = 0;
    let mut pos = 0;
    let mut num_reads = 0;
    let mut num_writes = 0;
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
                  tape[dest] = match num_reads {
                      0 => phase,
                      1 => inp,
                      _ => panic!("too many reads"),
                  };
                  num_reads += 1;
                  pos += 2},
            4 => {result = get_val(mode1, tape[pos+1], &tape);
                  // println!("output: {}", result);
                  num_writes += 1;
                  if num_writes > 1 {
                      panic!("too many writes");
                  }
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
    return result;
}

fn main() {
	let mut line = String::new();
	io::stdin().read_line(&mut line).expect("Failed to read line");
	let split = line.split(",").map(|x| x.trim().parse::<isize>().expect("failed to parse"));
	let tape: Vec<isize> = split.collect();
    let mut highest = 0;
    let mut highest_conf = (0, 0, 0, 0, 0);
    for i1 in 0..5 {
        let res1 = compute(i1, 0, &tape);
        for i2 in 0..5 {
            if i2 == i1 {
                continue;
            }
            let res2 = compute(i2, res1, &tape);
            for i3 in 0..5 {
                if i3 == i1 || i3 == i2 {
                    continue;
                }
                let res3 = compute(i3, res2, &tape);
                for i4 in 0..5 {
                    if i4 == i1 || i4 == i2 || i4 == i3 {
                        continue;
                    }
                    let res4 = compute(i4, res3, &tape);
                    for i5 in 0..5 {
                        if i5 == i1 || i5 == i2 || i5 == i3 || i5 == i4 {
                            continue;
                        }
                        let res5 = compute(i5, res4, &tape);
                        if highest < res5 {
                            highest = res5;
                            highest_conf = (i1, i2, i3, i4, i5);
                        }
                    }
                }
            }
        }
    }
    println!("{}", highest);
    println!("{} {} {} {} {}", highest_conf.0, highest_conf.1, highest_conf.2, highest_conf.3, highest_conf.4);
}
