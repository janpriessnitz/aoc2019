
use std::io;

fn fuel(mass: i32) -> i32 {
	let mut result: i32 = 0;
	let mut fuel_mass: i32 = mass/3 - 2;
	while	fuel_mass > 0 {
		result += fuel_mass;
		fuel_mass = fuel_mass/3 - 2;
	}
	result
}

fn main() {
	let mut result: i32 = 0;
	loop {
		let mass;
		let mut line = String::new();
		match io::stdin().read_line(& mut line) {
			Ok(0) => break,
			Err(_) => break,
			Ok(_) => mass = match line.trim().parse() {
				Ok(x) => x,
				Err(_) => {println!("wrong line {}", line); 0},
			},
		}
		result += fuel(mass);
	}
	println!("{}", result);
}
