const INPUT: &str = include_str!("../../inputs/2.in");

fn main() {
    // split line to vectors
    let lines: Vec<_> = INPUT
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    // let (mut first_elements, mut second_elements): (Vec<i32>, Vec<i32>) =
    // lines.into_iter().map(|pair| (pair[0], pair[1])).unzip();

    let mut safe_count = 0;
    for line in lines.clone() {
        let safe = is_safe(line);
        if safe {
            safe_count += 1
        }
    }
    // sort vectors
    // first_elements.sort();
    // second_elements.sort();
    println!("{:?}", safe_count)
}

fn is_safe(data: Vec<i32>) -> bool {
    if data.len() < 2 {
        return true; // nothing to compare, considered safe
    }

    let mut is_increase = None;

    for i in 1..data.len() {
        let diff = data[i] - data[i - 1];

        // diff greater than 3 then unsafe, return false
        if diff.abs() > 3 {
            return false;
        }

        // assign value for the first pair
        if i == 1 {
            is_increase = Some(diff > 0);
        } else if let Some(is_increase_val) = is_increase {
            let current_is_increase = diff > 0;
            // check that the incremental shoud be in the same direction as the previous one
            if is_increase_val != current_is_increase {
                return false;
            }
        }
    }
    // everything is ok, return true
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let result = is_safe(vec![7, 6, 4, 2, 1]);
        assert!(result);
    }
    #[test]
    fn two() {
        let result = is_safe(vec![1, 2, 7, 8, 9]);
        assert!(!result);
    }
    #[test]
    fn three() {
        let result = is_safe(vec![9, 7, 6, 2, 1]);
        assert!(!result);
    }
    #[test]
    fn four() {
        let result = is_safe(vec![1, 3, 2, 4, 5]);
        assert!(!result);
    }
    #[test]
    fn five() {
        let result = is_safe(vec![8, 6, 4, 4, 1]);
        assert!(!result);
    }
    #[test]
    fn six() {
        let result = is_safe(vec![1, 3, 6, 7, 9]);
        assert!(result);
    }
}

