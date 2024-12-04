use utils::read_file;
use regex::Regex;


const PART1_REGEX: &str = r#"(mul\(\d{1,3},\d{1,3}\))"#;
const PART2_REGEX: &str = r#"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don't\(\))"#;

fn part1<P: AsRef<str>>(p: P) -> u32 {
    let re = Regex::new(PART1_REGEX).unwrap();
    let content = read_file(p);

    let mut sum = 0;

    for cap in re.captures_iter(&content) {
        let mul = cap.get(0).unwrap().as_str();
        let (left, right) = mul.split_once(",").unwrap();
        let left = left.replace("mul(", "").parse::<u32>().unwrap();
        let right = right.replace(")", "").parse::<u32>().unwrap();

        sum += left * right;
    }

    sum
}

fn part2<P: AsRef<str>>(p: P) -> u32 {
    let re = Regex::new(PART2_REGEX).unwrap();
    let content = read_file(p);

    let mut sum = 0;

    let mut enabled = true;

    for cap in re.captures_iter(&content) {
        let op = cap.get(0).unwrap().as_str();

        if op.contains("don") {
            enabled = false;
        } else if op.contains("do") {
            enabled = true;
        } else if enabled {
            let (left, right) = op.split_once(",").unwrap();
            let left = left.replace("mul(", "").parse::<u32>().unwrap();
            let right = right.replace(")", "").parse::<u32>().unwrap();

            sum += left * right;
        }
    }

    sum
}

fn main() {
    println!("Part1: {0}", part1("../inputs/day3.txt"));
    println!("Part2: {0}", part2("../inputs/day3.txt"))
}
