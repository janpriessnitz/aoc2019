

use std::io;

fn get_val(mode: isize, parameter: isize, base: isize, tape: &Vec<isize>) -> isize {
    match mode {
        0 => return tape[parameter as usize],
        1 => return parameter,
        2 => return tape[(parameter + base) as usize],
        x => {println!("unknown mode {}", x); return 0},
    }
}

fn compute(inp: isize, tape: &mut Vec<isize>, pos: &mut usize, base: &mut isize) -> isize {
    let mut num_reads = 0;
    let mut num_writes = 0;
    loop {
        let mode1 = (tape[*pos] / 100) % 10;
        let mode2 = (tape[*pos] / 1000) % 10;
        let mode3 = (tape[*pos] / 10000) % 10;
        match tape[*pos] % 100 {
            1 => {let offset = if mode3 == 2 {*base} else {0};
                  let dest = (tape[*pos+3] + offset) as usize;
                  tape[dest] = get_val(mode1, tape[*pos+1], *base, &tape) + get_val(mode2, tape[*pos+2], *base, &tape);
                  *pos += 4; },
            2 => {let offset = if mode3 == 2 {*base} else {0};
                  let dest = (tape[*pos+3] + offset) as usize;
                  tape[dest] = get_val(mode1, tape[*pos+1], *base, &tape)*get_val(mode2, tape[*pos+2], *base, &tape);
                  *pos += 4; },
            3 => {let offset = if mode1 == 2 {*base} else {0};
                  let dest = (tape[*pos+1] + offset) as usize;
                  tape[dest] = match num_reads {
                      0 => inp,
                      _ => return -1,
                  };
                  num_reads += 1;
                  *pos += 2},
            4 => {let result = get_val(mode1, tape[*pos+1], *base, &tape);
                  // println!("output: {}", result);
                  *pos += 2;
                  return result;},
            5 => {match get_val(mode1, tape[*pos+1], *base, &tape) {
                    0 => {*pos += 3},
                    _ => {*pos = get_val(mode2, tape[*pos+2], *base, &tape) as usize},
                 }},
            6 => {match get_val(mode1, tape[*pos+1], *base, &tape) {
                    0 => {*pos = get_val(mode2, tape[*pos+2], *base, &tape) as usize},
                    _ => {*pos += 3},
                 }},
            7 => {let store_val;
                  if get_val(mode1, tape[*pos+1], *base, &tape) < get_val(mode2, tape[*pos+2], *base, &tape) {
                     store_val = 1;
                  } else {
                     store_val = 0;
                  }
                  let offset = if mode3 == 2 {*base} else {0};
                  let dest = (tape[*pos+3] + offset) as usize;
                  tape[dest] = store_val;
                  *pos += 4},
            8 => {let store_val;
                  if get_val(mode1, tape[*pos+1], *base, &tape) == get_val(mode2, tape[*pos+2], *base, &tape) {
                     store_val = 1;
                  } else {
                     store_val = 0;
                  }
                  let offset = if mode3 == 2 {*base} else {0};
                  let dest = (tape[*pos+3] + offset) as usize;
                  tape[dest] = store_val;
                  *pos += 4},
            9 => {*base += get_val(mode1, tape[*pos+1], *base, &tape);
                  *pos += 2},
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
	let source: Vec<isize> = split.collect();
    let mut tape = source.clone();
    let mut extension = vec![0; 100000];
    tape.append(&mut extension);
    let mut pos: usize = 0;
    let mut base: isize = 0;
    println!("starting");
    loop {
        println!("loop");
        let result = compute(2, &mut tape, &mut pos, &mut base);
        if result == -1000000000 {
            break
        }
        println!("output {}", result);
    }

}
