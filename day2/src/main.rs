use utils::read_lines;

fn get_input<P: AsRef<str>>(p: P) -> Vec<Vec<usize>> {
    read_lines(p)
        .iter()
        .map(|line| line.split_whitespace())
        .map(|split| split.map(|n| n.parse::<usize>().expect("should be a valid number")))
        .map(|split| split.collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn valid_measure(measure: &Vec<usize>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;
    let mut delta_valid = true;

    for i in 0..measure.len() - 1 {
        increasing &= measure[i] < measure[i + 1];
        decreasing &= measure[i] > measure[i + 1];

        let delta = measure[i].abs_diff(measure[i + 1]);
        delta_valid &= delta >= 1 && delta <= 3;
    }

    ((increasing || decreasing) && delta_valid)
}

fn part1<P: AsRef<str>>(p: P) -> usize {
    let report = get_input(p);

    let mut safe = 0;

    for measure in report {
        safe += valid_measure(&measure) as usize;
    }

    safe
}

fn part2<P: AsRef<str>>(p: P) -> Vec<Vec<usize>> {
    let report = get_input(p);

    let mut safe_vecs = vec![];

    for measure in report {
        let valid = valid_measure(&measure);
        if valid {
            safe_vecs.push(measure);
            continue;
        }

        // Can I remove a single index?
        for index in 0..measure.len() {
            let mut cloned = measure.clone();
            cloned.remove(index);

            if valid_measure(&cloned) {
                safe_vecs.push(measure);
                break;
            }
        }
    }

    safe_vecs
}

fn main() {
    println!("Number of safe reports: {0}", part1("../inputs/day2.txt"));

    let safe = part2("../inputs/day2.txt");
    println!(
        "Number of safe reports with dampening: {0}",
        safe.len(),
    );
}