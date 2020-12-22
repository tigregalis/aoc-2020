use std::collections::HashMap;

fn main() {
    let contents =
        std::fs::read_to_string("day10/input").expect("Something went wrong reading the file");

    // let contents = r#"
    // 16
    // 10
    // 15
    // 5
    // 1
    // 11
    // 7
    // 19
    // 6
    // 12
    // 4
    // "#
    // .to_string();

    // let contents = r#"
    // 28
    // 33
    // 18
    // 42
    // 31
    // 14
    // 46
    // 20
    // 48
    // 47
    // 24
    // 23
    // 49
    // 45
    // 19
    // 38
    // 39
    // 11
    // 1
    // 32
    // 25
    // 35
    // 8
    // 17
    // 7
    // 9
    // 4
    // 2
    // 34
    // 10
    // 3
    // "#
    // .to_string();

    println!("{}", contents);

    let mut adapters = contents
        .trim()
        .split_whitespace()
        .filter_map(|n| n.trim().parse::<usize>().ok())
        .collect::<Vec<_>>();

    adapters.push(0);

    adapters.push(adapters.iter().fold(0, |acc, a| acc.max(*a)) + 3);

    adapters.sort_unstable();

    println!("{:?}", adapters);

    let map = adapters
        .windows(2)
        .filter_map(|pair| {
            if let [a, b] = *pair {
                Some(b - a)
            } else {
                None
            }
        })
        .fold(HashMap::new(), |mut map, difference| {
            map.entry(difference)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            map
        });

    println!("{:?}", map);

    let answer = map.get(&3).unwrap() * map.get(&1).unwrap();

    println!("answer: {}", answer);

    // take adapters, SET(0)
    // if you exclude one number, is the difference of the numbers either side of it <=3?    while
    // queue = [[[0, 1, 2, 3, 4, 5, 6]]]
    // let mut queue = vec![(0usize, adapters)];
    // let mut possibilities = Vec::new();
    // let mut possibilities = HashSet::new();

    let diffs = adapters
        .windows(2)
        .filter_map(|pair| {
            if let [a, b] = *pair {
                Some(b - a)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    println!("{:?}", diffs);

    let ranges = diffs
        .split(|n| *n == 3usize)
        .filter_map(|range| {
            if range.is_empty() {
                None
            } else {
                Some(range.iter().sum::<usize>())
            }
        })
        .collect::<Vec<_>>();

    println!("ranges: {:?}", ranges);

    let combinations = ranges
        .iter()
        .map(|range| match *range {
            // f(1) = 1 = 1
            // [1]
            1 => 1,

            // f(2) = 2 = 1 + 1
            // [1, 1]
            //   [2]
            2 => 2,

            // f(3) = 4 = 1 + 2 + 1
            // [1, 1, 1]
            //   [2, 1]
            //     [3]
            //   [1, 2]
            3 => 4,

            // f(4) = 7 = 1 + 3 + 3
            // [1, 1, 1, 1]
            //   [2, 1, 1]
            //     [3, 1]
            //     [2, 2]
            //   [1, 2, 1]
            //     [1, 3]
            //   [1, 1, 2]
            4 => 7,

            // f(5) = 11 = 1 + 4 + 4 + 2
            // [1, 1, 1, 1, 1]
            //   [2, 1, 1, 1]
            //     [3, 1, 1]
            //       [3, 2]
            //     [2, 2, 1]
            //       [2, 3]
            //   [1, 2, 1, 1]
            //     [1, 3, 1]
            //     [1, 2, 2]
            //   [1, 1, 2, 1]
            //   [1, 1, 1, 2]
            5 => 11,

            // f(6) = 17 = 1 + 4 + 7 + 4 + 1
            // [1, 1, 1, 1, 1, 1]
            //   [2, 1, 1, 1, 1]
            //     [3, 1, 1, 1]
            //       [3, 2, 1]
            //         [3, 3]
            //     [2, 2, 1, 1]
            //       [2, 3, 1]
            //     [2, 1, 2, 1]
            //   [1, 2, 1, 1, 1]
            //     [1, 3, 1, 1]
            //       [1, 3, 2]
            //     [1, 2, 2, 1]
            //       [1, 2, 3]
            //   [1, 1, 2, 1, 1]
            //     [1, 1, 3, 1]
            //   [1, 1, 1, 2, 1]
            //     [1, 1, 1, 3]
            6 => 17,

            // not done
            _ => todo!("unrecognised value: {}", *range),
        })
        .collect::<Vec<_>>();

    println!("combinations: {:?}", combinations);

    let possibilities = combinations.iter().fold(1u64, |acc, n| acc * *n);

    println!("possibilities: {}", possibilities);
}
