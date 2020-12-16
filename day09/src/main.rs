// The debug version
#[cfg(debug_assertions)]
macro_rules! debug_println {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

// Non-debug version
#[cfg(not(debug_assertions))]
macro_rules! debug_println {
    ($( $args:expr ),*) => {};
}
fn main() {
    let contents =
        std::fs::read_to_string("day09/input").expect("Something went wrong reading the file");
    const PREAMBLE: usize = 25;
    const RANGE: usize = 25;

    // let contents = r#"
    // 35
    // 20
    // 15
    // 25
    // 47
    // 40  # [35, 20, 15, 25, 47]
    //     # ignore [47] entirely as they're larger than 40 [35, 20, 15, 25]
    //     # ignore 20 entirely as it's exactly half of 40 (check only for even numbers, or 40 % 2 == 0) [35, 15, 25]
    //     # check 35, 40 - 35 = 5, are any of the other [15, 25] numbers 5? no, also exclude 35 as we've already checked it [15, 25]
    //     # check 15, 40 - 15 = 25, are any of the other [25] numbers 25? yes, so this number is valid
    // 62  # [20, 15, 25, 47, 40]
    //     # check 20, 62 - 20 = 42, are any of the other [15, 25, 47, 40] numbers 42? no, also exclude 20 as we've already checked it [15, 25, 47, 40]
    //     # check 15, 62 - 15 = 47, are any of the other [25, 47, 40] numbers 47? yes, so this number is valid
    // 55  # ignore 62 entirely as it's larger than 55
    //     # check 15, 55 - 15 = 40, are any of the other numbers 40? yes, so this number is valid
    // 65  # check 25, 65 - 25 = 40, are any of the other numbers 40? yes, so this number is valid
    // 95  # check 47, 95 - 47 = 48, are any of the other numbers 48? no, also exclude 47 as we've already checked it
    //     # check 40, 95 - 40 = 55, are any of the other numbers
    // 102 # ...
    // 117 # ...
    // 150 # ...
    // 182 # ...
    // 127 # [95, 102, 117, 150, 182]
    //     # ignore [150, 182] entirely as they're larger than 127 [95, 102, 117]
    //     # check 95, 127 - 95 = 32, are any of the other [102, 117] numbers 32? no, also exclude 95 as we've already checked it [102, 117]
    //     # check 102, 127 - 102 = 25, are any of the other [117] numbers 25? no, also exclude 25 as we've already checked it [117]
    //     # no point checking 117, there's only one number left <- this is the number that doesn't match
    // 219
    // 299
    // 277
    // 309
    // 576
    //     "#
    // .to_string()
    // .trim()
    // .split('\n')
    // .map(|line| line.split('#').next().unwrap().trim())
    // .filter(|line| !line.is_empty())
    // .collect::<Vec<_>>()
    // .join("\n");
    // const PREAMBLE: usize = 5;
    // const RANGE: usize = 5;

    let numbers = contents
        .trim()
        .split('\n')
        .map(|number| number.parse::<usize>().expect("Should be a number"))
        .collect::<Vec<_>>();

    let impostor = numbers
        .iter()
        .enumerate()
        .skip(PREAMBLE)
        .find(|(idx, target)| {
            let subset = &numbers[(*idx - RANGE)..*idx];
            debug_println!("target: {}", target);
            debug_println!("  before filter: {:?}", subset);
            let subset = subset
                .iter()
                .filter(|n| {
                    if n > target {
                        debug_println!("    ignore {} entirely as it's larger than {}", n, target);
                        false
                    } else {
                        true
                    }
                })
                .filter(|n| {
                    if **n % 2 == 0 && **n * 2 == **target {
                        debug_println!(
                            "    ignore {} entirely as it's exactly half of {}",
                            n,
                            target
                        );
                        false
                    } else {
                        true
                    }
                })
                .collect::<Vec<_>>();
            debug_println!("  after filter: {:?}", subset);
            let mut subset = subset.iter();
            while let Some(n) = subset.next() {
                let diff = *target - *n;
                debug_println!(
                    "    check {}: {} - {} = {} ... are any of the other {:?} numbers {}?",
                    n,
                    target,
                    n,
                    diff,
                    subset.clone().collect::<Vec<_>>(),
                    diff
                );
                if subset.clone().any(|m| **m == diff) {
                    debug_println!("      yes");
                    break;
                } else {
                    debug_println!("      no");
                }
            }
            subset.count() == 0
        });

    if let Some(impostor) = impostor {
        println!("we have an impostor: {}", impostor.1);

        let target = impostor.1;

        if let Some((min, max)) = numbers.iter().enumerate().find_map(|(idx, _number)| {
            let mut i = 0;
            let sum = loop {
                i += 1;
                let sum = numbers[idx..idx + i].iter().sum::<usize>();
                if sum >= *target {
                    break sum;
                }
            };
            if sum == *target {
                let (min, max) = numbers[idx..idx + i]
                    .iter()
                    .fold((usize::MAX, usize::MIN), |(min, max), number| {
                        (min.min(*number), max.max(*number))
                    });
                Some((min, max))
            } else {
                None
            }
        }) {
            println!("min {} + max {} = {}", min, max, min + max);
        }
    }
}
