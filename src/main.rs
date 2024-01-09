use std::fs::read_to_string;

type Lens = (String, i32);
type Boxes = Vec<Vec<Lens>>;

const EMPTY_BOX: Vec<Lens> = Vec::new();

const REMOVE: char = '-';
const ADD: char = '=';

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
    let mut boxes: Boxes = vec![EMPTY_BOX; 256];
    for lens in lenses {
        let label = lens.chars().take_while(|c| *c != REMOVE && *c != ADD).collect::<String>();
        let box_number = hash(&label) as usize;
        if lens.chars().find(|c| *c == ADD).is_some() {
            let focal_length = lens
                .chars()
                .skip_while(|c| !c.is_numeric())
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            if let Some(index) = boxes[box_number].iter().position(|l| l.0 == label) {
                boxes[box_number][index].1 = focal_length;
            } else {
                boxes[box_number].push((label, focal_length));
            }
        } else if lens.chars().find(|c| *c == REMOVE).is_some() {
            if let Some(index) = boxes[box_number].iter().position(|l| l.0 == label) {
                boxes[box_number].remove(index);
            }
        } else {
            panic!("Invalid lens: {}", lens);
        }
    }
    boxes
}

fn solution_2(filename: &str) -> i32 {
    let lenses = get_sequence(filename);
    let boxes: Boxes = organize_lenses(&lenses);
    total_focusing_power(boxes)
}

fn main() {
    println!("{}", solution("example.txt"));
    println!("{}", solution("input.txt"));
    println!("{}", solution_2("example.txt"));
    println!("{}", solution_2("input.txt"));
}
