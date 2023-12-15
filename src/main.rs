use std::fs::read_to_string;

type Lens = (String, i32);
type Boxes = &'static [Vec<&'static Lens>; 255];

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

fn total_focusing_power(boxes: Boxes) -> i32 {
    let mut sum = 0;
    for (box_number, box_) in boxes.iter().enumerate() {
        for (slot, lens) in box_.iter().enumerate() {
            sum += (box_number as i32 + 1) * (slot as i32 + 1) * lens.1;
        }
    }
    sum
}

fn organize_lenses(lenses: &Vec<String>) -> Boxes {
  // TODO
}

fn solution_2(filename: &str) -> i32 {
    let lenses = get_sequence(filename);
    let boxes: Boxes = organize_lenses(&lenses);
    total_focusing_power(&boxes)
}

fn main() {
    assert_eq!(solution("example.txt"), 1320);
    assert_eq!(solution("input.txt"), 520500);
    assert_eq!(solution_2("example.txt"), 145);
    assert_eq!(solution_2("input.txt"), 0);
}
