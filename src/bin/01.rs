use ac::read_lines;

fn main() {
    let mut sum = 0;
    if let Ok(lines) = read_lines("inputs/1.in") {
        for line in lines {
            let result:u32 = calibrate(&line.unwrap());
            sum += result;
        }
    }
    println!("{sum}");
}

fn calibrate(word: &str) -> u32{
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;

    for char in word.chars() {
        if char.is_numeric() {
            if first.is_none() {
                first = Some(char);
            }
            last = Some(char);
        }
    }

    let mut number = first.unwrap().to_string();
    number.push(last.unwrap());
    number.parse().unwrap()
}
