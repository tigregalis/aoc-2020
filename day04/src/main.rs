use lazy_static::lazy_static;
use regex::Regex;

struct Field<'a> {
    name: &'a str,
    validator: fn(&str) -> bool,
}

fn main() {
    let contents =
        std::fs::read_to_string("day04/input").expect("Something went wrong reading the file");

    let required_fields = [
        Field {
            // (Birth Year)
            name: "byr",
            validator: |value| {
                value.len() == 4
                    && if let Ok(year) = value.parse::<i32>() {
                        (1920..=2002).contains(&year)
                    } else {
                        false
                    }
            },
        },
        Field {
            // (Issue Year)
            name: "iyr",
            validator: |value| {
                value.len() == 4
                    && if let Ok(year) = value.parse::<i32>() {
                        (2010..=2020).contains(&year)
                    } else {
                        false
                    }
            },
        },
        Field {
            // (Expiration Year)
            name: "eyr",
            validator: |value| {
                value.len() == 4
                    && if let Ok(year) = value.parse::<i32>() {
                        (2020..=2030).contains(&year)
                    } else {
                        false
                    }
            },
        },
        Field {
            // (Height)
            name: "hgt",
            validator: |value| {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"(?P<height>\d+)(?P<dim>cm|in)").unwrap();
                }
                matches!(
                    RE.captures(value).map(|capture| {
                        if let Ok(height) = &capture["height"].parse::<i32>() {
                            match &capture["dim"] {
                                "cm" => (150..=193).contains(height),
                                "in" => (59..=76).contains(height),
                                _ => false,
                            }
                        } else {
                            dbg!(value);
                            false
                        }
                    }),
                    Some(true)
                )
            },
        },
        Field {
            // (Hair Color)
            name: "hcl",
            validator: |value| {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
                }
                RE.is_match(value)
            },
        },
        Field {
            // (Eye Color)
            name: "ecl",
            validator: |value| {
                const ECL: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                ECL.contains(&value)
            },
        },
        Field {
            // (Passport ID)
            name: "pid",
            validator: |value| {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
                }
                RE.is_match(value)
            },
        },
    ];

    let passports = contents
        .split("\n\n")
        .map(|passport| passport.split_whitespace().map(|pairs| pairs.split(':')));

    let count = passports
        .clone()
        .filter(|passport| {
            let fields = passport.clone().map(|mut pairs| pairs.next().unwrap());
            required_fields.iter().all(|required_field| {
                fields
                    .clone()
                    .any(|field_name| field_name == required_field.name)
            })
        })
        .count();

    eprintln!("count: {}", count);

    let count = passports
        .clone()
        .filter(|passport| {
            let fields = passport.clone().map(|mut pairs| pairs.next().unwrap());
            required_fields.iter().all(|required_field| {
                fields
                    .clone()
                    .any(|field_name| field_name == required_field.name)
            })
        })
        .filter(|passport| {
            let mut fields = passport
                .clone()
                .map(|mut pairs| (pairs.next().unwrap(), pairs.next().unwrap()));
            println!();
            fields.all(|(field_name, field_value)| {
                if let Some(required_field) = required_fields
                    .iter()
                    .find(|required_field| field_name == required_field.name)
                {
                    if (required_field.validator)(field_value) {
                        eprintln!("o -- {}:{}", field_name, field_value);
                        true
                    } else {
                        eprintln!("x -- {}:{}", field_name, field_value);
                        false
                    }
                } else {
                    true
                }
            })
        })
        .count();

    eprintln!("count: {}", count);
}
