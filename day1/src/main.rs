use std::env;
use std::fs;

fn part1() {
    let sample = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    let args: Vec<String> = env::args().collect();

    let mut contents: String = sample.to_string();
    if args.len() > 1 {
        let file = &args[1];
        contents = fs::read_to_string(file).expect("Failed to read file.");
    }

    let lines: Vec<&str> = contents.lines().collect();

    let mut numbers: Vec<u32> = Vec::new();
    for line in lines {
        let l = line.trim();
        let mut num = String::new();
        // TODO This would have been better
        // l.matches(char::is_numeric).collect();
        for c in l.chars() {
            match c {
                c if c.is_digit(10) => {
                    num.push(c);
                }
                _ => continue,
            }
        }
        let first = &num[0..1];
        let last = &num[num.len() - 1..];
        println!("{} -> {first} & {last}", num);
        numbers.push(
            format!("{}{}", first, last)
                .parse()
                .expect("Failed to parse number"),
        );
    }

    let mut sum = 0;
    for n in numbers {
        sum += n;
    }
    println!("Sum is {sum}");
}

fn part2() {
    let sample = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";

    let args: Vec<String> = env::args().collect();

    let mut contents: String = sample.to_string();
    if args.len() > 1 {
        let file = &args[1];
        contents = fs::read_to_string(file).expect("Failed to read file.");
    }

    let lines: Vec<&str> = contents.lines().collect();

    let replacements = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    let mut numbers: Vec<u32> = Vec::new();
    for line in lines {
        let l = line.trim();

        let mut num = String::new();
        for (i, c) in l.chars().enumerate() {
            match c {
                c if c.is_digit(10) => {
                    num.push(c);
                    continue;
                }
                _ => {}
            }

            let slice = &l[i..];
            for (from, to) in replacements {
                if slice.starts_with(from) {
                    num.push(to);
                    break;
                }
            }
        }

        let first = &num[0..1];
        let last = &num[num.len() - 1..];
        println!("{} -> {first} & {last}", num);
        numbers.push(
            format!("{}{}", first, last)
                .parse()
                .expect("Failed to parse number"),
        );
    }

    let mut sum = 0;
    for n in numbers {
        sum += n;
    }
    println!("Sum is {sum}");
}

fn main() {
    // part1();
    part2();
}
