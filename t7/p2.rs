

use std::io;
use std::cmp;

fn get_val(mode: isize, parameter: isize, tape: &Vec<isize>) -> isize {
    match mode {
        0 => return tape[parameter as usize],
        1 => return parameter,
        x => {println!("unknown mode {}", x); return 0},
    }
}

fn compute(inp: isize, tape: &mut Vec<isize>, pos: &mut usize) -> isize {
    let mut result = 0;
    let mut num_reads = 0;
    let mut num_writes = 0;
    loop {
        let mode1 = (tape[*pos] / 100) % 10;
        let mode2 = (tape[*pos] / 1000) % 10;
        let mode3 = (tape[*pos] / 10000) % 10;
        match tape[*pos] % 100 {
            1 => {let dest = tape[*pos+3] as usize;
                  tape[dest] = get_val(mode1, tape[*pos+1], &tape) + get_val(mode2, tape[*pos+2], &tape);
                  *pos += 4; },
            2 => {let dest = tape[*pos+3] as usize;
                  tape[dest] = get_val(mode1, tape[*pos+1], &tape)*get_val(mode2, tape[*pos+2], &tape);
                  *pos += 4; },
            3 => {let dest = tape[*pos+1] as usize;
                  tape[dest] = match num_reads {
                      0 => inp,
                      _ => return -1,
                  };
                  num_reads += 1;
                  *pos += 2},
            4 => {result = get_val(mode1, tape[*pos+1], &tape);
                  // println!("output: {}", result);
                  *pos += 2;
                  return result;},
            5 => {match get_val(mode1, tape[*pos+1], &tape) {
                    0 => {*pos += 3},
                    _ => {*pos = get_val(mode2, tape[*pos+2], &tape) as usize},
                 }},
            6 => {match get_val(mode1, tape[*pos+1], &tape) {
                    0 => {*pos = get_val(mode2, tape[*pos+2], &tape) as usize},
                    _ => {*pos += 3},
                 }},
            7 => {let store_val;
                  if get_val(mode1, tape[*pos+1], &tape) < get_val(mode2, tape[*pos+2], &tape) {
                     store_val = 1;
                  } else {
                     store_val = 0;
                  }
                  let dest = tape[*pos+3] as usize;
                  tape[dest] = store_val;
                  *pos += 4},
            8 => {let store_val;
                  if get_val(mode1, tape[*pos+1], &tape) == get_val(mode2, tape[*pos+2], &tape) {
                     store_val = 1;
                  } else {
                     store_val = 0;
                  }
                  let dest = tape[*pos+3] as usize;
                  tape[dest] = store_val;
                  *pos += 4},
            99 => break,
            x => {println!("unknown op {}", x); break},
        }
    }
    return -1000000000;
}

fn main() {
	let mut line = String::new();
	io::stdin().read_line(&mut line).expect("Failed to read line");
	let split = line.split(",").map(|x| x.trim().parse::<isize>().expect("failed to parse"));
	let tape: Vec<isize> = split.collect();
    let mut highest = 0;
    let mut highest_conf = (0, 0, 0, 0, 0);
    for i1 in 5..10 {
        for i2 in 5..10 {
            if i2 == i1 {
                continue;
            }
            for i3 in 5..10 {
                if i3 == i1 || i3 == i2 {
                    continue;
                }
                for i4 in 5..10 {
                    if i4 == i1 || i4 == i2 || i4 == i3 {
                        continue;
                    }
                    for i5 in 5..10 {
                        if i5 == i1 || i5 == i2 || i5 == i3 || i5 == i4 {
                            continue;
                        }
                        let mut tape1 = tape.clone();
                        let mut tape2 = tape.clone();
                        let mut tape3 = tape.clone();
                        let mut tape4 = tape.clone();
                        let mut tape5 = tape.clone();
                        let mut res1 = 0;
                        let mut res2 = 0;
                        let mut res3 = 0;
                        let mut res4 = 0;
                        let mut res5 = 0;
                        let mut pos1 = 0;
                        let mut pos2 = 0;
                        let mut pos3 = 0;
                        let mut pos4 = 0;
                        let mut pos5 = 0;
                        let mut result = 0;

                        // println!("{}", compute(i1, &mut tape1, &mut pos1));
                        // println!("{}", compute(i2, &mut tape2, &mut pos2));
                        // println!("{}", compute(i3, &mut tape3, &mut pos3));
                        // println!("{}", compute(i4, &mut tape4, &mut pos4));
                        // println!("{}", compute(i5, &mut tape5, &mut pos5));
                        compute(i1, &mut tape1, &mut pos1);
                        compute(i2, &mut tape2, &mut pos2);
                        compute(i3, &mut tape3, &mut pos3);
                        compute(i4, &mut tape4, &mut pos4);
                        compute(i5, &mut tape5, &mut pos5);
                        loop {
                            result = res5;
                            res1 = compute(res5, &mut tape1, &mut pos1);
                            res2 = compute(res1, &mut tape2, &mut pos2);
                            res3 = compute(res2, &mut tape3, &mut pos3);
                            res4 = compute(res3, &mut tape4, &mut pos4);
                            res5 = compute(res4, &mut tape5, &mut pos5);
                            if res5 == -1000000000 {
                                break;
                            }
                        }
                        println!("{}", result);
                        if highest < result {
                            highest = result;
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
