const INPUT: &str = include_str!("../../inputs/1.in");
fn main() {
    let lines: Vec<_> = INPUT
        .lines()
        .enumerate()
        .map(|(_x, line)| {
            line.split_whitespace()
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let (mut first_elements, mut second_elements): (Vec<_>, Vec<_>) =
        lines.into_iter().map(|pair| (pair[0], pair[1])).unzip();

    // sort vectors
    first_elements.sort();
    second_elements.sort();

    // let a = map.iter();
    let mut diffs: Vec<i32> = Vec::new();
    for i in 0..first_elements.len() {
        let diff = first_elements[i] - second_elements[i];
        let diff_abs = diff.abs();
        diffs.push(diff_abs)
    }
    let sum = diffs.iter().sum::<i32>();
    println!("{:?}", sum);

    // let (first_elements, second_elements): (Vec<_>, Vec<_>) =
    //     map.into_iter().map(|pair| (pair[0], pair[1])).unzip();

    // let output = part1(input);
    // println!("{output}");
}

fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let result: u32 = calibrate(line);
        sum += result;
    }
    sum
}

fn calibrate(word: &str) -> u32 {
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;

    for char in word.chars() {
        if char.is_numeric() {
            first = Some(char);
            break;
        }
    }

    for char in word.chars().rev() {
        if char.is_numeric() {
            last = Some(char);
            break;
        }
    }

    let mut number = first.unwrap().to_string();
    number.push(last.unwrap());
    number.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet",
        );
        assert_eq!(result, 142);
    }
}
