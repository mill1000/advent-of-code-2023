use std::env;
use std::fs;
use std::collections::HashSet;

fn part1(lines: Vec<&str>) {
    let mut scores = Vec::new();
    for line in lines {
        let (_card, result) = line.split_once(":").unwrap();

        let (winners, numbers) = result.split_once("|").unwrap();

        let winners: HashSet<u32> = winners.trim().split_whitespace().map(|x: &str| x.parse::<u32>().unwrap() ).collect();
        let numbers: HashSet<u32> = numbers.trim().split_whitespace().map(|x: &str| x.parse::<u32>().unwrap() ).collect();

        // let winner_set = winners.intersection(&numbers).collect::<HashSet<_>>();
        let win_count:u32 = winners.intersection(&numbers).collect::<HashSet<_>>().len().try_into().unwrap();

        let points = if win_count > 0 {2_u32.pow(win_count - 1)} else {0};
        println!("Win count {win_count} Points: {points}");
 
        scores.push(points);
    }
    let sum: u32 = scores.iter().sum();
    println!("Part1 sum is {sum}");
}

fn part2(lines: Vec<&str>) {
    let mut cards = 0;
    let mut multipliers: Vec<i32> = Vec::new();
    for line in lines {
        let (_card, result) = line.split_once(":").unwrap();

        let (winners, numbers) = result.split_once("|").unwrap();

        let winners: HashSet<u32> = winners.trim().split_whitespace().map(|x: &str| x.parse::<u32>().unwrap() ).collect();
        let numbers: HashSet<u32> = numbers.trim().split_whitespace().map(|x: &str| x.parse::<u32>().unwrap() ).collect();

        // let winner_set = winners.intersection(&numbers).collect::<HashSet<_>>();
        let win_count:i32 = winners.intersection(&numbers).collect::<HashSet<_>>().len().try_into().unwrap();

        // let points = if win_count > 0 {2_u32.pow(win_count - 1)} else {0};
        // println!("Win count {win_count} Multipliers: {:?}", multipliers);
 
        let multi = multipliers.iter().filter(|x| **x > 0).count();

        multipliers = multipliers.iter().filter_map(|x| match *x - 1 {
            x if x.is_positive() => Some(x),
            _ => None
        }).collect();
        // println!("Multi {multi} Multipliers new: {:?}", multipliers);

        
        for _ in 0..(multi+1) {
            // println!("Push");
            cards += 1;
            multipliers.push(win_count);
        }

    }

    println!("Part2 is {cards}");
}

fn main() {
    let sample = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
  
    let args: Vec<String> = env::args().collect();

    let mut contents: String = sample.to_string();
    if args.len() > 1 {
        let file = &args[1];
        contents = fs::read_to_string(file).expect("Failed to read file.");
    }

    let lines: Vec<&str> = contents.lines().collect();
    
    // part1(lines);
    part2(lines);
}
