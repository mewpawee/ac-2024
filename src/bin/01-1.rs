fn main() {
    let input = include_str!("../../inputs/1.in");
    let output = part1(input);
    println!("{output}");
}

fn  part1(input:&str) -> u32 {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let result:u32 = calibrate(line);
        sum += result;
    };
    sum
}

fn calibrate(word: &str) -> u32{
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
        treb7uchet"
        );
        assert_eq!(result,142);
    }
}
