// use lazy_static::lazy_static;
use regex::Regex;

use std::collections::{HashMap, HashSet};

fn main() {
    let contents =
        std::fs::read_to_string("day07/input").expect("Something went wrong reading the file");

    //     let contents = r#"
    // light red bags contain 1 bright white bag, 2 muted yellow bags.
    // dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    // bright white bags contain 1 shiny gold bag.
    // muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    // shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    // dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    // vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    // faded blue bags contain no other bags.
    // dotted black bags contain no other bags.
    // "#
    //     .trim()
    //     .to_string();

    //     let contents = r#"
    // shiny gold bags contain 2 dark red bags.
    // dark red bags contain 2 dark orange bags.
    // dark orange bags contain 2 dark yellow bags.
    // dark yellow bags contain 2 dark green bags.
    // dark green bags contain 2 dark blue bags.
    // dark blue bags contain 2 dark violet bags.
    // dark violet bags contain no other bags.
    //     "#
    //     .trim()
    //     .to_string();

    let re_outer = Regex::new(
        r#"(?x) # ignore whitespace
            (?P<outer_colour>\w+\s\w+) # "light red" colour of the outer bag
            \s
            bags # "bags"
            \s
            contain # "contain"
            \s
            (?P<contents>.*) # "the contents"
            \.
            "#,
    )
    .unwrap();

    let re_inner = Regex::new(
        r#"(?x) # ignore whitespace
            (?P<quantity>\d+) # "1" quantity of the inner bag
            \s
            (?P<colour>\w+\s\w+) # "bright white" colour of the inner bag
            \s
            bags? # "bag"
            "#,
    )
    .unwrap();

    let bag_holds = contents
        .trim()
        .split('\n')
        .map(|rule| {
            let outer_bag = re_outer.captures(rule).unwrap();
            let outer_colour = &outer_bag["outer_colour"];
            let contents = &outer_bag["contents"];
            let inner_rules = if contents == "no other bags" {
                HashMap::new()
            } else {
                let contents_rules = contents.split(", ").map(|content| {
                    let inner_bag = re_inner.captures(content).unwrap();
                    let quantity = &inner_bag["quantity"];
                    let colour = &inner_bag["colour"];
                    (colour.to_owned(), quantity.parse::<i32>().unwrap())
                });
                contents_rules.collect::<HashMap<_, _>>()
            };
            (outer_colour.to_owned(), inner_rules)
        })
        .collect::<HashMap<_, _>>();

    let mut queue = vec!["shiny gold"];
    let mut set = HashSet::new();
    while !queue.is_empty() {
        queue = bag_holds
            .iter()
            .filter_map(|(outer, inners)| {
                if inners
                    .iter()
                    .any(|(inner, _)| queue.contains(&inner.as_str()))
                {
                    set.insert(outer.to_string());
                    Some(outer.as_str())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
    }

    let total = set.len();

    println!("total possible unique holders of a shiny gold: {}", total);

    let mut queue: Vec<(&str, i32)> = vec![("shiny gold", 1)];
    let mut sum = Vec::new();
    while !queue.is_empty() {
        queue = queue
            .iter()
            .flat_map(|(key_outer, count_outer)| {
                sum.push(*count_outer);
                bag_holds
                    .get(*key_outer)
                    .unwrap()
                    .iter()
                    .map(move |(key, count)| (key.as_str(), count_outer * *count))
            })
            .collect::<Vec<_>>();
    }

    let total = sum.iter().sum::<i32>() - 1;

    println!("total bags held by 1 shiny gold: {}", total);
}
