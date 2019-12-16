

use std::io;

fn eval_tape(mut tape: Vec<usize>) -> usize {
	let mut pos = 0;
	loop {
		let result;
		match tape[pos] {
			1 => result = tape[tape[pos+1]] + tape[tape[pos+2]],
			2 => result = tape[tape[pos+1]] * tape[tape[pos+2]],
			99 => break,
			x => {println!("unknown op {}", x); break},
		}
		let destindex = tape[pos+3];
		tape[destindex] = result;
		pos += 4;
	}
	tape[0]
}


fn main() {
	let mut line = String::new();
	io::stdin().read_line(&mut line).expect("Failed to read line");
	let split = line.split(",").map(|x| x.trim().parse::<usize>().expect("failed to parse"));
	let init_tape: Vec<usize> = split.collect();
	for a in 0..100 {
		for b in 0..100 {
			let mut cur_tape = init_tape.clone();
			cur_tape[1] = a;
			cur_tape[2] = b;
			println!("{} {} {}", a, b, eval_tape(cur_tape));
		}
	}

}
