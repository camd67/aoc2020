use regex::{Regex, Match};

#[allow(dead_code)]
pub fn solve_day2() {
    let raw_passwords = include_str!("./data/day2.txt");

    let password_matcher = Regex::new(r"(\d+)-(\d+) ([a-z]): (.+)")
        .expect("Failed to compile password matching regex");

    part1_and_2(raw_passwords, password_matcher);
}

struct Password {
    requirement_one: u8,
    requirement_two: u8,
    required_char: u8,
    pass: String,
}

impl Password {
    fn is_valid_old_policy(&self) -> bool {
        let mut char_count = 0;

        for c in self.pass.as_bytes() {
            if self.required_char.eq(c) {
                char_count += 1;
            }
        }

        char_count >= self.requirement_one && char_count <= self.requirement_two
    }

    fn is_valid_new_policy(&self) -> bool {
        let pass_bytes = self.pass.as_bytes();
        let matches_first = pass_bytes[self.requirement_one as usize - 1] == self.required_char;
        let matches_second = pass_bytes[self.requirement_two as usize - 1] == self.required_char;
        (matches_first || matches_second) && (matches_first != matches_second)
    }
}

fn parse_match_as_u8(matched: Option<Match>) -> u8 {
    matched.map(|m| m.as_str().parse::<u8>().unwrap()).unwrap()
}

fn part1_and_2(raw_passwords: &str, password_matcher: Regex) {
    let mut valid_count = 0;
    let mut valid_count2 = 0;

    for line in raw_passwords.lines() {
        // A better way to do this would be to scan through the line, switching on the character
        // and splitting the line that way.
        // However... I thought of that after doing a bunch of this problem and tbh I don't want
        // to rewrite it all...

        let captured = password_matcher.captures(line).expect("Unable to parse password line");
        let password = Password {
            requirement_one: parse_match_as_u8(captured.get(1)),
            requirement_two: parse_match_as_u8(captured.get(2)),
            // Wow this is kinda gross?
            required_char: captured.get(3).unwrap().as_str().as_bytes()[0],
            pass: captured.get(4).map(|s| String::from(s.as_str())).unwrap(),
        };

        if password.is_valid_old_policy() {
            valid_count += 1;
        }
        if password.is_valid_new_policy() {
            valid_count2 += 1;
        }
    }

    println!("Valid old password count: {}", valid_count);
    println!("Valid new password count: {}", valid_count2);
}
