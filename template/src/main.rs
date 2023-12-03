use std::env;
use std::fs;

fn part1(lines: Vec<&str>) {
}

fn part2(lines: Vec<&str>) {
}

fn main() {
    let sample = "";
  
    let args: Vec<String> = env::args().collect();

    let mut contents: String = sample.to_string();
    if args.len() > 1 {
        let file = &args[1];
        contents = fs::read_to_string(file).expect("Failed to read file.");
    }

    let lines: Vec<&str> = contents.lines().collect();
    
    // part1(lines);
    // part2(lines);
}
