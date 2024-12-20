use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/5.in");

fn main() {
    // parse rules_raw and updates_raw
    let (rules_raw, updates_raw) = INPUT.split("\n\n").take(2).collect_tuple().unwrap();
    let mut rules_forward = HashMap::new();
    let mut rules_backward = HashMap::new();
    for line in rules_raw.lines() {
        if let Ok(values) = line
            .split('|')
            .map(str::trim)
            .map(str::parse::<i32>)
            .collect::<Result<Vec<i32>, _>>()
        {
            if values.len() >= 2 {
                rules_forward.insert(values[0], values[1]);
                rules_backward.insert(values[1], values[0]);
            }
        }
    }

    // for (key, value) in &rules {
    //     println!("{}: {}", key, value);
    // }

    let updates: Vec<Vec<i32>> = updates_raw
        .lines()
        .map(|line| {
            line.split(",")
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    check_rules(updates[0].clone(), rules_forward, rules_backward);
}

fn check_rules(
    update: Vec<i32>,
    rules_forward: HashMap<i32, i32>,
    rules_backward: HashMap<i32, i32>,
) -> bool {
    for number in update {
        // check before
        let before = rules_backward.get(&number).unwrap();
        println!("{:?}", before);
        // check after
        let after = rules_forward.get(&number).unwrap();
        println!("{:?}", after);
    }
    true
}
