use std::fs::read_to_string;

fn get_sequence(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .split(",")
        .map(|line| line.to_string())
        .collect()
}

fn hash(input: &str) -> i32 {
    input.chars().fold(0, |acc, c| ((acc + c as i32) * 17) % 256)
}

fn solution(filename: &str) -> i32 {
    let sequence = get_sequence(filename);
    let mut sum = 0;
    for step in sequence {
        sum += hash(&step);
    }
    sum
}

fn main() {
    assert_eq!(solution("example.txt"), 1320);
    assert_eq!(solution("input.txt"), 520500);
}
