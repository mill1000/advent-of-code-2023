use std::env;
use std::fs;
use std::iter::zip;

fn part1(lines: &Vec<&str>) {
    // D = Tt - t^2

    let times: Vec<u32> = lines[0]
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let records: Vec<u32> = lines[1]
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    // dbg!(records);

    let mut win_count = Vec::new();
    for (time, record) in zip(times, records) {
        println!("Time {time}, Record: {record} mm");

        let wins: Vec<u32> = (0..=time)
            .filter_map(|t| {
                let d = (time * t) - t * t;
                // println!("Time {t}, Dist: {d} mm");
                if d > record {
                    Some(d)
                } else {
                    None
                }
            })
            .collect();
        // dbg!(wins);

        win_count.push(wins.len());
    }

    let product: usize = win_count.iter().product();
    println!("Part 1 result {product}");
}

fn part2(lines: &Vec<&str>) {
    let times = [lines[0]
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap()];
    let records = [lines[1]
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap()];
    // dbg!(records);

    let mut win_count = Vec::new();
    for (time, record) in zip(times, records) {
        println!("Time {time}, Record: {record} mm");

        let wins: Vec<u64> = (0..=time)
            .filter_map(|t| {
                let d = (time * t) - t * t;
                // println!("Time {t}, Dist: {d} mm");
                if d > record {
                    Some(d)
                } else {
                    None
                }
            })
            .collect();
        // dbg!(wins);

        win_count.push(wins.len());
    }

    let result = win_count.first().unwrap();
    println!("Part 2 result {result}");
}

fn main() {
    let sample = "Time:      7  15   30
Distance:  9  40  200";

    let args: Vec<String> = env::args().collect();

    let mut contents: String = sample.to_string();
    if args.len() > 1 {
        let file = &args[1];
        contents = fs::read_to_string(file).expect("Failed to read file.");
    }

    let lines: Vec<&str> = contents.lines().collect();

    part1(&lines);
    part2(&lines);
}
