
use std::io;
use std::collections::HashMap;


fn parse_and_insert(orbit: &str, orbits: &mut HashMap<String, Vec<String>>) {
    let mut split = orbit.trim().split(")");
    let key = match split.next() {
        Some(v) => v.to_string(),
        None => panic!(),
    };
    let val = match split.next() {
        Some(v) => v.to_string(),
        None => panic!(),
    };
    orbits.entry(key).or_insert_with(Vec::new).push(val);
}

fn get_num_orbits(planet: &String, depth: usize, orbits: & HashMap<String, Vec<String>>) -> usize {
    let mut result: usize = depth;
    match orbits.get(planet) {
        Some(v) => for p in v {
            result += get_num_orbits(p, depth + 1, orbits);
        },
        None => {},
    }
    return result;
}

fn main() {
    let mut orbits = HashMap::new();
    let mut line = String::new();
    loop {
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => parse_and_insert(&line, &mut orbits),
            Err(_) => {println!("fail!"); break;},
        }
        line.clear();
    }
    let com = "COM".to_string();
    println!("{}", get_num_orbits(&com, 0, &orbits));

}
