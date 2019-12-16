
use std::io;
use std::collections::HashMap;
use std::cmp;


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

fn get_distance(planet: &String, orbits: & HashMap<String, Vec<String>>) -> (usize, usize) {
    let mut you = 1000000000;
    let mut santa = 1000000000;
    if planet.as_str() == "YOU" {
        return (0, 1000000000);
    }
    if planet.as_str() == "SAN" {
        return (1000000000, 0);
    }
    match orbits.get(planet) {
        Some(v) => for p in v {
            let (cy, cs) = get_distance(p, orbits);
            you = cmp::min(you, cy);
            santa = cmp::min(santa, cs);
        },
        None => {},
    }
    if you < 1000000000 && santa < 1000000000 {
        return (you, santa);
    } else {
        return (you+1, santa+1);
    }
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
    let (santa, you) = get_distance(&com, &orbits);
    println!("{}", santa + you);

}
