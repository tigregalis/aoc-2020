use std::str::Lines;

fn main() {
    let contents =
        std::fs::read_to_string("day03/input").expect("Something went wrong reading the file");
    let rows = contents.lines();

    let [right, down] = [3, 1];
    let count = |rows: Lines, right: usize, down: usize| {
        rows.step_by(down)
            .enumerate()
            .filter(|(y, row)| matches!(row.chars().nth((y * right) % row.len()), Some('#')))
            .count()
    };
    println!("count: {}", count(rows.clone(), right, down));

    let product = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]]
        .iter()
        .map(|[right, down]| count(rows.clone(), *right, *down))
        .product::<usize>();

    println!("product: {}", product);
}
