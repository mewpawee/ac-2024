fn main() {
    let input = include_str!("../../inputs/1.in");
    let output = part2(input);
    dbg!(output);
}

fn  part2(input:&str) -> u32 {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let result:u32 = calibrate(line);
        sum += result;
    };
    sum
}

fn calibrate(word: &str) -> u32{
    let word_copy = word;
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;

    let word_copy = word_copy
        .replace("one", "one1one")
        .replace("two","two2two")
        .replace("three","three3three")
        .replace("four","four4four")
        .replace("five","five5five")
        .replace("six","six6six")
        .replace("seven","seven7seven")
        .replace("eight","eight8eight")
        .replace("nine","nine9nine");

    for char in word_copy.chars() {
        if char.is_numeric() {
            first = Some(char);
            break;
        }
    }

    for char in word_copy.chars().rev() {
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
        let result = part2(
        "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"
        );
        assert_eq!(result,281);
    }

    #[test]
    fn one_word() {
        let result = calibrate(
        "two1nine"
        );
        assert_eq!(result,29);
    }
}
