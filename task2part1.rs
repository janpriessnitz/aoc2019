

use std::io;

fn main() {
	let mut line = String::new();
	io::stdin().read_line(&mut line).expect("Failed to read line");
	let split = line.split(",").map(|x| x.trim().parse::<usize>().expect("failed to parse"));
	let mut tape: Vec<usize> = split.collect();
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
	println!("position 0: {}", tape[0]);
	println!("position 3: {}", tape[3]);

}
