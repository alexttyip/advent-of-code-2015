use crypto::digest::Digest;
use crypto::md5::Md5;
use std::fs;
use std::time::Instant;

type Int = u64;
type InputType = String;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day04/input.txt")
        .unwrap()
        .trim()
        .to_string()
}

fn solve(input: InputType) -> (Int, Int) {
    let mut hasher = Md5::new();
    let key = input.as_bytes();

    let mut part1 = 0;

    for i in 0.. {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let mut digest = [0; 16];
        hasher.result(&mut digest);

        if part1 == 0 && digest[..2] == [0, 0] && digest[2] <= 0x0F {
            part1 = i;
        }

        if digest[..3] == [0, 0, 0] {
            return (part1, i);
        }

        hasher.reset();
    }

    panic!()
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

    assert_eq!(part1, 346386);
    assert_eq!(part2, 9958218);
}
