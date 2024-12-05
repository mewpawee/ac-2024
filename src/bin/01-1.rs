const INPUT: &str = include_str!("../../inputs/1.in");

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

    let (mut first_elements, mut second_elements): (Vec<i32>, Vec<i32>) =
        lines.into_iter().map(|pair| (pair[0], pair[1])).unzip();

    // sort vectors
    first_elements.sort();
    second_elements.sort();

    // part 1
    // sum diff
    let mut diffs: Vec<i32> = Vec::new();
    for i in 0..first_elements.len() {
        let diff = first_elements[i] - second_elements[i];
        let diff_abs = diff.abs();
        diffs.push(diff_abs)
    }
    let diffs_sum = diffs.iter().sum::<i32>();
    println!("diffs: {:?}", diffs_sum);

    // part2
    let mut similars: Vec<i32> = Vec::new();

    for first_element in first_elements.clone() {
        let mut count = 0;
        for second_element in second_elements.clone() {
            if first_element == second_element {
                count += 1;
            }
        }
        similars.push(first_element * count)
    }
    let similars_sum = similars.iter().sum::<i32>();
    println!("similars: {:?}", similars_sum);
}

fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let result: u32 = 0;
        sum += result;
    }
    sum
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

