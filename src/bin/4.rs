const INPUT: &str = include_str!("../../inputs/4.in");
use regex::Regex;

fn main() {
    let lines: Vec<Vec<char>> = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let count = check_count(lines.clone());
    // println!("{:?}", count)
}

fn check_count(lines: Vec<Vec<char>>) -> i32 {
    let mut all_count = 0;
    all_count += check_horizontal(lines.clone());
    all_count += check_vertical(lines.clone());
    all_count += check_diagonal_1(lines.clone());
    all_count
}

fn check_horizontal(lines: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for line in lines {
        for j in 0..line.len() - 4 {
            let sub_string: String = line[j..j + 4].iter().collect();
            match sub_string.as_str() {
                "XMAS" => count += 1,
                "SAMX" => count += 1,
                _ => {}
            }
        }
    }
    count
}

fn check_vertical(lines: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    // println!("{:?}", lines[0].len());
    for i in 0..lines[0].len() {
        // println!("{:?}", lines.len());
        for j in 0..lines.len() - 4 {
            // println!("{:?}, {:?}", i, j);
            // let sub_string: String = lines[j - 4..j][i].iter().collect();

            let sub_string: String = [
                lines[j][i],
                lines[j + 1][i],
                lines[j + 2][i],
                lines[j + 3][i],
            ]
            .iter()
            .collect();
            match sub_string.as_str() {
                "XMAS" => count += 1,
                "SAMX" => count += 1,
                _ => {}
            }
        }
    }
    count
}

fn check_diagonal_1(lines: Vec<Vec<char>>) -> i32 {
    let (m, n) = (lines.len(), lines[0].len());
    let mut result: Vec<String> = vec![];
    // traverse top-right
    for d in 0..n {
        let (mut row, mut coll) = (0, d);
        let mut internal_result: Vec<char> = vec![];
        while row < m && coll > 0 {
            // println!("{:?} {:?}", row, coll);
            internal_result.push(lines[row][coll]);
            row += 1;
            coll -= 1;
        }
        result.push(internal_result.iter().collect())
    }
    // traverse bottom-left
    for d in 1..m {
        let (mut row, mut coll) = (d, n - 1);
        let mut internal_result: Vec<char> = vec![];
        while row < m && coll > 0 {
            internal_result.push(lines[row][coll]);
            row += 1;
            coll -= 1;
        }
        result.push(internal_result.iter().collect())
    }
    let mut count = 0;
    let re = Regex::new(r"XMAS|SAMX").unwrap();
    for sub_string in result {
        count += re.find_iter(&sub_string).count();
    }
    println!("{:?}", count);
    count as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lines: Vec<Vec<char>> = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        let result = check_count(lines);
        assert_eq!(result, 18);
    }
}

