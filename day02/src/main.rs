use regex::Regex;

fn main() {
    let contents =
        std::fs::read_to_string("day02/input").expect("Something went wrong reading the file");

    let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<letter>\w): (?P<pass>\w+)").unwrap();
    let captures = contents.lines().map(|line| {
        let captures = re.captures(line).unwrap();
        let min = captures["min"].parse::<usize>().unwrap();
        let max = captures["max"].parse::<usize>().unwrap();
        let letter = captures["letter"].chars().next().unwrap();
        let pass = &captures["pass"];
        // can this be done without cloning pass?
        (min, max, letter, pass.to_owned())
    });

    let count = captures
        .clone()
        .filter(|(min, max, letter, pass)| {
            let count = pass.chars().filter(|chr| chr == letter).count();
            (min..=max).contains(&&count)
        })
        .count();
    println!("count: {}", count);

    let count = captures
        .clone()
        .filter(|(min, max, letter, pass)| {
            let set = [min, max];
            pass.char_indices()
                .filter(|&(index, chr)| set.contains(&&(index + 1)) && chr == *letter)
                .count()
                == 1
        })
        .count();
    println!("count: {}", count);
}
