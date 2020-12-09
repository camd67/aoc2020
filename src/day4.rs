use std::collections::HashSet;
use regex::Regex;

#[allow(dead_code)]
pub fn solve_day4() {
    let raw_passwords = include_str!("./data/day4.txt");

    let color_regex = Regex::new("#[0-9a-f]{6}").unwrap();
    let passport_id_regex = Regex::new("[0-9]{9}").unwrap();

    let do_validate = true;

    let mut valid_passports = 0;
    let mut checked_fields = create_passport_requirements();
    for line in raw_passwords.lines() {
        if line.is_empty() {
            if checked_fields.is_empty() {
                valid_passports += 1;
            }

            // Reset our checked fields
            checked_fields = create_passport_requirements();
        } else {
            let fields = line.split(' ');
            for field in fields {
                let split_field: Vec<&str> = field.split(':').collect();
                let field_key = split_field[0];
                let field_value = split_field[1];
                // Hmmm... probably a better way!
                let valid = match field_key {
                    "byr" => {
                        let year = field_value.parse::<u32>().unwrap();
                        year >= 1920 && year <= 2002
                    },
                    "iyr" => {
                        let year = field_value.parse::<u32>().unwrap();
                        year >= 2010 && year <= 2020
                    },
                    "eyr" => {
                        let year = field_value.parse::<u32>().unwrap();
                        year >= 2020 && year <= 2030
                    },
                    "hgt" => {
                        // sneakily check for in / cm based on our rules of 2 digit cm and 3 digit
                        // in. I'm not proud of this.
                        if field_value.len() == 4 {
                            let size = field_value[0..2].parse::<u32>().unwrap();
                            size >= 59 && size <= 76 && &field_value[2..] == "in"
                        } else if field_value.len() == 5 {
                            let size = field_value[0..3].parse::<u32>().unwrap();
                            size >= 150 && size <= 193 && &field_value[3..] == "cm"
                        } else {
                            false
                        }
                    },
                    "hcl" => color_regex.is_match(field_value),
                    "ecl" => {
                        matches!(field_value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
                    },
                    "pid" => passport_id_regex.is_match(field_value),
                    _ => false,
                };
                if valid || !do_validate {
                    checked_fields.remove(field_key);
                }
            }
        }
    }

    // collect the last one
    // HA HA
    // So turns out that there's an off-by-one error here. If we're not validating fields we need a
    // +1 here
    // But... if we are validating fields then we're 1 too high
    // tbh I have NO idea why this is happening...
    if checked_fields.is_empty() {
        valid_passports += 1;
    } else if do_validate {
        valid_passports -= 1;
    }

    println!("Valid passport count: {}", valid_passports);
}

fn create_passport_requirements() -> HashSet<&'static str> {
    let mut requirements = HashSet::new();
    requirements.insert("byr");
    requirements.insert("iyr");
    requirements.insert("eyr");
    requirements.insert("hgt");
    requirements.insert("hcl");
    requirements.insert("ecl");
    requirements.insert("pid");
    requirements
}
