use std::collections::HashMap;
use utils::read_lines;

fn read_lists<P: AsRef<str>>(p: P) -> (Vec<u32>, Vec<u32>) {
    let lines = read_lines(p);
    let mut left_locations = Vec::with_capacity(lines.len());
    let mut right_locations = Vec::with_capacity(lines.len());

    for line in lines {
        let (left, right) = line.split_once("   ").unwrap();

        left_locations.push(left.parse::<u32>().unwrap());
        right_locations.push(right.parse::<u32>().unwrap());
    }

    (left_locations, right_locations)
}

/// Solve part 1, run time is O(nlog(n)), memory is O(n)
fn part1<P: AsRef<str>>(p: P) -> u32 {
    let (mut left_locations, mut right_locations) = read_lists(p);

    // O(nlog(n))
    left_locations.sort();
    right_locations.sort();

    let mut distance = 0;

    for (left, right) in left_locations.iter().zip(right_locations) {
        distance += left.abs_diff(right);
    }

    distance
}

/// Solve part 2, runtime is O(n), amortized
fn part2<P: AsRef<str>>(p: P) -> u32 {
    let (left_locations, right_locations) = read_lists(p);
    let mut rhs_map: HashMap<u32, u32> = HashMap::new();

    // Surely there's a one-liner for this
    for location in right_locations {
        let entry = rhs_map.get_mut(&location);
        match entry {
            None => {rhs_map.insert(location, 1);},
            Some(n) => *n += 1
        }
    }

    let mut similarity = 0;

    for location in left_locations {
        similarity += location * rhs_map.get(&location).unwrap_or(&0);
    }

    similarity
}

fn main() {
    println!("Distance: {0}", part1("../inputs/day1.txt"));
    println!("Similarity score: {0}", part2("../inputs/day1.txt"));
}

#[cfg(test)]
mod tests {
    use crate::{part2};

    #[test]
    fn test_part_2() {
        dbg!(part2("../inputs/day1_ex.txt"));
    }

}

