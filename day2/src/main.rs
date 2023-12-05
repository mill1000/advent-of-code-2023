use std::cmp;
use std::collections::HashMap;
use std::env;
use std::fs;

fn part1(lines: Vec<&str>) {
    let mut good_games: Vec<u32> = Vec::new();
    let mut powers: Vec<u32> = Vec::new();
    for line in lines {
        // println!("line is {}", line);
        let (game, result) = line.split_once(":").expect("Failed to split on ':'.");

        let (_, id) = game.trim().split_once(" ").unwrap();
        let id: u32 = id.parse().unwrap();
        println!("Game ID {id}");

        let mut max_colors = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        for set in result.split(";") {
            // println!("{}", set);
            for (count, color) in set.split(",").map(|x| x.trim().split_once(" ").unwrap()) {
                println!("Got {count} of {}", color);
                let count: u32 = count.parse().unwrap();
                max_colors.insert(color, cmp::max(count, max_colors[color]));
            }
        }

        // dbg!(&max_colors);
        powers.push(max_colors.values().product());
        if max_colors["red"] > 12 || max_colors["green"] > 13 || max_colors["blue"] > 14 {
            continue;
        }

        good_games.push(id);
    }
    // dbg!(good_games);
    let sum: u32 = good_games.iter().sum();
    println!("Part1 sum is {sum}");

    // dbg!(powers);
    let sum_p2: u32 = powers.iter().sum();
    println!("Part2 sum is {}", sum_p2);
}

// fn _part2(lines: Vec<&str>) {}

fn main() {
    let sample = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let args: Vec<String> = env::args().collect();

    let mut contents: String = sample.to_string();
    if args.len() > 1 {
        let file = &args[1];
        contents = fs::read_to_string(file).expect("Failed to read file.");
    }

    let lines: Vec<&str> = contents.lines().collect();

    part1(lines);
    // part2(lines);
}
