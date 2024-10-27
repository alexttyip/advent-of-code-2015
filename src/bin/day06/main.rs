use itertools::Itertools;
use std::fs;
use std::time::Instant;

type Int = usize;
type InputType = Vec<(u8, Int, Int, Int, Int)>;

fn read_input() -> InputType {
    let file = fs::read_to_string("./src/bin/day06/input.txt").unwrap();

    file.trim()
        .lines()
        .flat_map(|s| {
            let words = s.split(' ').collect_vec();

            let (op, i, ii) = if words[0] == "toggle" {
                (2, 1, 3)
            } else if words[1] == "on" {
                (1, 2, 4)
            } else {
                (0, 2, 4)
            };

            let (x, y): (Int, Int) = words[i]
                .split(',')
                .flat_map(|i| i.parse())
                .collect_tuple()?;

            let (xx, yy): (Int, Int) = words[ii]
                .split(',')
                .flat_map(|i| i.parse())
                .collect_tuple()?;

            Some((op, x, xx, y, yy))
        })
        .collect()
}

fn solve(input: InputType) -> (usize, u32) {
    let mut part1 = [[false; 1000]; 1000];
    let mut part2 = [[0; 1000]; 1000];

    for (op, x, xx, y, yy) in input {
        for i in x..=xx {
            for j in y..=yy {
                match op {
                    0 => {
                        part1[i][j] = false;

                        if part2[i][j] > 0 {
                            part2[i][j] -= 1;
                        }
                    }
                    1 => {
                        part1[i][j] = true;
                        part2[i][j] += 1;
                    }
                    2 => {
                        part1[i][j] ^= true;
                        part2[i][j] += 2;
                    }
                    _ => panic!(),
                };
            }
        }
    }

    (
        part1.iter().flatten().filter(|b| **b).count(),
        part2.iter().flatten().sum(),
    )
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

    assert_eq!(part1, 569999);
    assert_eq!(part2, 17836115);
}
