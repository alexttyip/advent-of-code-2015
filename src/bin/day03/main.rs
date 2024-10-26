use num::Complex;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

type Int = i32;
type InputType = String;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day03/input.txt").unwrap()
}

const N: Complex<Int> = Complex::new(1, 0);
const E: Complex<Int> = Complex::new(0, 1);
const S: Complex<Int> = Complex::new(-1, 0);
const W: Complex<Int> = Complex::new(0, -1);

fn solve(input: InputType) -> (usize, usize) {
    let mut santa = Complex::new(0, 0);
    let mut robot = Complex::new(0, 0);

    let mut part1 = HashSet::from([santa]);
    let mut part2 = HashSet::from([robot]);

    for (i, ch) in input.chars().enumerate() {
        let d = match ch {
            '^' => N,
            '>' => E,
            'v' => S,
            '<' => W,
            _ => panic!(),
        };

        if i % 2 == 0 {
            santa += d;
            part2.insert(santa);
        } else {
            robot += d;
            part2.insert(robot);
        }

        part1.insert(santa + robot);
    }

    (part1.len(), part2.len())
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

    assert_eq!(part1, 2565);
    assert_eq!(part2, 2639);
}
