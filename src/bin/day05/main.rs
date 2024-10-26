use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

type Int = u16;
type InputType = Vec<String>;

fn read_input() -> InputType {
    fs::read_to_string("./src/bin/day05/input.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_owned())
        .collect()
}

fn check_vowel(ch: char) -> bool {
    matches!(ch, 'a' | 'e' | 'i' | 'o' | 'u')
}

fn part1(input: InputType) -> Int {
    let mut ans = 0;

    for s in input {
        let mut vowel_count = check_vowel(s.chars().next().unwrap()) as Int;
        let mut has_double = false;
        let mut has_forbidden = false;

        for (ch1, ch2) in s.chars().tuple_windows() {
            if check_vowel(ch2) {
                vowel_count += 1
            }

            has_double |= ch1 == ch2;

            if matches!(
                (ch1, ch2),
                ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')
            ) {
                has_forbidden = true;
                break;
            }
        }

        if vowel_count >= 3 && has_double && !has_forbidden {
            ans += 1;
        }
    }

    ans
}

fn part2(input: InputType) -> Int {
    let mut ans = 0;

    let mut map: HashMap<_, Vec<_>> = HashMap::new();

    for s in input {
        let mut a = false;

        for (i, (ch1, ch2)) in s.chars().tuple_windows().enumerate() {
            let v = map.entry((ch1, ch2)).or_default();

            if v.iter().any(|j| i - j >= 2) {
                a = true;
                break;
            }

            v.push(i);
        }

        map.clear();

        if !a {
            continue;
        }

        for (ch1, _, ch2) in s.chars().tuple_windows() {
            if ch1 == ch2 {
                ans += 1;
                break;
            }
        }
    }

    ans
}

pub fn main() {
    let mut now = Instant::now();
    let input = read_input();
    let input_elapsed = now.elapsed();

    now = Instant::now();
    let part1 = part1(input.clone());
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2(input);
    let part2_elapsed = now.elapsed();

    println!("--- Day 05 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Reading input took: {:.2?}", input_elapsed);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 238);
    assert_eq!(part2, 69);
}
