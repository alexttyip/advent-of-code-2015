use itertools::Itertools;
use std::fs;
use std::time::Instant;

type Int = u32;
type InputType = Vec<(Int, Int, Int)>;

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day02/input.txt").unwrap();

    file.trim()
        .lines()
        .flat_map(|s| s.split('x').flat_map(|s| s.parse().ok()).collect_tuple())
        .collect()
}

fn solve(input: InputType) -> (Int, Int) {
    input
        .iter()
        .fold((0, 0), |(mut part1, mut part2), (l, w, h)| {
            let a1 = l * w;
            let a2 = w * h;
            let a3 = h * l;

            part1 += (a1 + a2 + a3) * 2 + a1.min(a2).min(a3);
            part2 += (l + w + h - l.max(w).max(h)) * 2 + l * w * h;

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

    assert_eq!(part1, 1598415);
    assert_eq!(part2, 3812909);
}
