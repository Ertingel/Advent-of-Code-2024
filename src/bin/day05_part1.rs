// cargo run  --bin day05_part1
// cargo test --bin day05_part1

use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../././input/day05.txt");
    let output = solve(input);
    println!("Day05 part1: {output}");
}

fn parse_rules(data: &str) -> HashMap<i32, HashSet<i32>> {
    let entries: Vec<(i32, i32)> = data
        .lines()
        .map(|rule| {
            let values: Vec<i32> = rule
                .split('|')
                .map(|page| page.parse::<i32>().unwrap())
                .collect();
            (values[0], values[1])
        })
        .collect();

    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();

    for (prerequisite, page) in entries {
        let prerequisites = rules.get_mut(&page);

        if let Some(prerequisites) = prerequisites {
            prerequisites.insert(prerequisite);
        } else {
            let mut prerequisites: HashSet<i32> = HashSet::new();
            prerequisites.insert(prerequisite);
            rules.insert(page, prerequisites);
        }
    }

    rules
}

fn parse_updates(data: &str) -> Vec<Vec<i32>> {
    data.lines()
        .map(|update| {
            update
                .split(',')
                .map(|page| page.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn validate_update(rules: &HashMap<i32, HashSet<i32>>, update: &Vec<i32>) -> bool {
    let all_pages: HashSet<i32> = HashSet::from_iter(update.clone());
    let mut printed: HashSet<i32> = HashSet::new();

    for page in update {
        let prerequisites = rules.get(page);
        if let Some(prerequisites) = prerequisites {
            let prerequisites: HashSet<i32> =
                prerequisites.intersection(&all_pages).cloned().collect();

            if !printed.is_superset(&prerequisites) {
                return false;
            }
        }
        printed.insert(*page);
    }

    true
}

fn solve(input: &str) -> i32 {
    let data: Vec<&str> = input.split("\n\n").collect();

    let rules = parse_rules(data[0]);
    let updates = parse_updates(data[1]);

    let valid: Vec<&Vec<i32>> = updates
        .iter()
        .filter(|update| validate_update(&rules, update))
        .collect();

    let center: Vec<i32> = valid
        .iter()
        .map(|update| update[update.len() / 2])
        .collect();

    // Getting total.
    let total: i32 = center.iter().sum();

    /*
    println!("{:?}", rules);
    println!("{:?}", updates);
    println!("{:?}", valid);
    println!("{:?}", center);
    println!("{total}");
    /*
    75,47,61,53,29 -> Valid   -> 61
    97,61,53,29,13 -> Valid   -> 53
    75,29,13       -> Valid   -> 29
    75,97,47,61,53 -> Invalid
    61,13,29       -> Invalid
    97,13,75,29,47 -> Invalid

    (61 + 53 + 29) = 143
    */
    */

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05_part1() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let output = solve(input);
        assert_eq!(output, 143)
    }
}
