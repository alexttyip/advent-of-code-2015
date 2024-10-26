use std::fs;
use std::time::Instant;

type InputType = String;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day01/input.txt").unwrap()
}

fn solve(input: InputType) -> (i64, usize) {
    input
        .chars()
        .enumerate()
        .fold((0, 0), |(mut part1, mut part2), (i, c)| {
            part1 += if c == '(' { 1 } else { -1 };

            if part2 == 0 && part1 == -1 {
                part2 = i + 1;
            }

            (part1, part2)
        })
}

pub fn main() {
    let mut now = Instant::now();
    let input = read_input();
    let input_elapsed = now.elapsed();

    now = Instant::now();
    let (part1, part2) = solve(input);
    let solve_elapsed = now.elapsed();

    println!("--- Day 01 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Solve took: {:.2?}", solve_elapsed);

    assert_eq!(part1, 74);
    assert_eq!(part2, 1795);
}
