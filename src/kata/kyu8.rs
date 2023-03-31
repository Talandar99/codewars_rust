pub fn number_to_string(i: i32) -> String {
    format!("{}", i)
}

pub fn boolean_to_string(b: bool) -> String {
    b.to_string()
}

pub fn opposite(number: i32) -> i32 {
    std::ops::Neg::neg(number)
}

pub fn abbrev_name(name: &str) -> String {
    let splited_names = name.split_whitespace();
    let mut initials: String = "".to_string();
    for splited_name in splited_names {
        let temp = splited_name
            .chars()
            .nth(0)
            .unwrap()
            .to_uppercase()
            .to_string();
        initials.push_str(&temp);
        initials.push_str(".");
    }
    initials.remove(3);
    return initials.to_string();
}

pub fn count_by(x: u32, n: u32) -> Vec<u32> {
    let mut number: Vec<u32> = Vec::new();
    for i in 1..=n {
        number.push(i * x);
    }
    return number;
}

pub fn get_grade(s1: u16, s2: u16, s3: u16) -> char {
    let score: u16 = (s1 + s2 + s3) / 3;
    match score {
        90..=100 => return 'A',
        80..=89 => return 'B',
        70..=79 => return 'C',
        60..=69 => return 'D',
        _ => return 'F',
    }
}

pub fn rps(p1: &str, p2: &str) -> &'static str {
    return match p1 {
        "rock" => match p2 {
            "paper" => "Player 2 won!",
            "rock" => "Draw!",
            _ => "Player 1 won!",
        },
        "paper" => match p2 {
            "rock" => "Player 1 won!",
            "paper" => "Draw!",
            _ => "Player 2 won!",
        },
        _ => match p2 {
            "rock" => "Player 2 won!",
            "paper" => "Player 1 won!",
            _ => "Draw!",
        },
    };
}
