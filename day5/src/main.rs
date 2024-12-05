use std::collections::HashMap;
use utils::read_lines;

fn part1<P: AsRef<str>>(p: P) -> usize {
    let lines = read_lines(p);

    let split = lines.splitn(2, |line| line.len() == 0).collect::<Vec<_>>();
    assert_eq!(2, split.len());

    let rules = split[0];
    let updates = split[1];

    // Table of rules: page => [pages you must be before]
    let mut rules_table = HashMap::new();
    for rule in rules {
        let (before, after) = rule.split_once("|").unwrap();
        let before = before.parse::<usize>().unwrap();
        let after = after.parse::<usize>().unwrap();

        rules_table
            .entry(before)
            .and_modify(|before_these: &mut Vec<usize>| before_these.push(after))
            .or_insert(vec![after]);
    }

    // All the updates as number
    let mut update_numbers = Vec::with_capacity(updates.len());
    // All the rules as Map page->index
    let mut update_mapping = Vec::with_capacity(updates.len());
    for update in updates {
        let numbers_map = update
            .split(",")
            .map(|num| num.parse::<usize>().unwrap())
            .enumerate()
            .map(|(i, number)| (number, i))
            .collect::<Vec<_>>();

        let mut page_map: HashMap<usize, usize> = HashMap::new();
        for (page, index) in &numbers_map {
            page_map.insert(*page, *index);
        }
        update_mapping.push(page_map);
        let as_list = numbers_map
            .into_iter()
            .map(|(page, _)| page)
            .collect::<Vec<_>>();
        update_numbers.push(as_list);
    }

    let mut sum = 0;
    for (update, mapping) in update_numbers.iter().zip(update_mapping) {
        let mut ok = true;
        for (i, page) in update.iter().enumerate() {
            if let Some(rules) = rules_table.get(page) {
                ok &= rules
                    .iter()
                    .flat_map(|page| mapping.get(page))
                    .all(|index| *index > i);
            }
        }

        if ok {
            let update_len = update.len();
            let midpoint = update_len / 2;
            sum += update[midpoint];
        }
    }

    sum
}

fn main() {
    println!("[1] Sum of midpoints: {0}", part1("../inputs/day5.txt"))
}
