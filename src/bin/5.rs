use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/5.in");

fn main() {
    let result = solve(INPUT);
    println!("{:?}", result);
}

fn solve(input: &str) -> i32 {
    // parse rules_raw and updates_raw
    let (rules_raw, updates_raw) = input.split("\n\n").take(2).collect_tuple().unwrap();
    let mut rules_forward = Vec::new();
    let mut rules_backward = Vec::new();

    for line in rules_raw.lines() {
        if let Ok(values) = line
            .split('|')
            .map(str::trim)
            .map(str::parse::<i32>)
            .collect::<Result<Vec<i32>, _>>()
        {
            if values.len() >= 2 {
                rules_forward.push((values[0], values[1]));
                rules_backward.push((values[1], values[0]));
            }
        }
    }

    let updates: Vec<Vec<i32>> = updates_raw
        .lines()
        .map(|line| {
            line.split(",")
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut pass_rules = Vec::new();

    for update in updates.into_iter() {
        let result = check_rules(
            update.clone(),
            rules_forward.clone(),
            rules_backward.clone(),
        );
        if result {
            pass_rules.push(update);
        }
    }

    // map middle
    let middle_sum: i32 = pass_rules.iter().map(|v| v[v.len() / 2]).sum();

    middle_sum
}

fn check_rules(
    update: Vec<i32>,
    rules_forward: Vec<(i32, i32)>,
    rules_backward: Vec<(i32, i32)>,
) -> bool {
    for (i, number) in update.iter().enumerate() {
        // let [before_elements, after_elements] = update.clone().splice(i);
        let before_elements = &update[..i];
        let after_elements = &update[i + 1..];

        let filtered_forwards: Vec<i32> = rules_forward
            .iter()
            .filter(|(key, _value)| *key == *number)
            .map(|(_key, value)| *value)
            .collect();
        let filtered_backwards: Vec<i32> = rules_backward
            .iter()
            .filter(|(key, _value)| *key == *number)
            .map(|(_key, value)| *value)
            .collect();

        for element in before_elements {
            for forward in &filtered_forwards {
                if *forward == *element {
                    return false;
                }
            }
        }

        for element in after_elements {
            for backward in &filtered_backwards {
                if *backward == *element {
                    return false;
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solve(
            "47|53
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
97,13,75,29,47
",
        );
        assert_eq!(result, 143);
    }
}

