use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/5.in");

fn main() {
    // parse rules_raw and updates_raw
    let (rules_raw, updates_raw) = INPUT.split("\n\n").take(2).collect_tuple().unwrap();
    // parse rules
    let rules: Vec<_> = rules_raw
        .lines()
        .map(|line| {
            line.split("|")
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    println!("{:?}", rules[0][1]);
    let updates: Vec<_> = updates_raw
        .lines()
        .map(|line| {
            line.split(",")
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    println!("{:?}", updates[0]);
    // let mut texts: Vec<&str> = vec![];
    // // capture mul(number,number), do() and don't
    // let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    // for cap in re.find_iter(INPUT) {
    //     let text = cap.as_str();
    //     texts.push(text);
    // }

    // let mut is_ok = true;
    // let mut sum = 0;

    // for text in texts {
    //     // let text = "mul(125,904)";
    //     match parse_text(text) {
    //         Cmds::Mul(a, b) => {
    //             if is_ok {
    //                 sum += a * b
    //             }
    //         }
    //         Cmds::Do => is_ok = true,
    //         Cmds::Dont => is_ok = false,
    //         Cmds::Invalid => println!("Something went wrong, this case shouldn't happen"),
    //     }
    // }
    // println!("sum: {:?}", sum);
}

enum Cmds {
    Mul(i32, i32),
    Do,
    Dont,
    Invalid, // Add a variant to handle invalid input
}

fn parse_text(input: &str) -> Cmds {
    if let Some(stripped) = input.strip_prefix("mul(") {
        if let Some(stripped) = stripped.strip_suffix(')') {
            let parts: Vec<&str> = stripped.split(',').collect();
            if parts.len() == 2 {
                if let (Ok(num1), Ok(num2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    return Cmds::Mul(num1, num2);
                }
            }
        }
    } else if input == "do()" {
        return Cmds::Do;
    } else if input == "don't()" {
        return Cmds::Dont;
    }
    Cmds::Invalid // Return Invalid for any non-matching input
}
