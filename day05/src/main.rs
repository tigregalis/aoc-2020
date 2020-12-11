fn main() {
    let contents =
        std::fs::read_to_string("day05/input").expect("Something went wrong reading the file");

    let ids = contents
        .lines()
        .map(|line| {
            let (rows, columns) = line.split_at(7);
            (
                rows.chars()
                    .fold((0, 127), |(min, max), chr| {
                        let mid = (min + max) / 2;
                        match chr {
                            'F' => (min, mid),
                            'B' => (mid + 1, max),
                            _ => unreachable!(),
                        }
                    })
                    .0,
                columns
                    .chars()
                    .fold((0, 7), |(min, max), chr| {
                        let mid = (min + max) / 2;
                        match chr {
                            'L' => (min, mid),
                            'R' => (mid + 1, max),
                            _ => unreachable!(),
                        }
                    })
                    .0,
            )
        })
        .map(|(row, column)| (row, column, row * 8 + column));

    ids.clone()
        .max_by_key(|(_row, _column, id)| *id)
        .iter()
        .for_each(|(row, column, id)| {
            println!("max id: row {}, column {}, id {}", row, column, id);
        });

    let min_row = ids
        .clone()
        .min_by_key(|(row, _column, _id)| *row)
        .unwrap()
        .0
        + 1;
    let max_row = ids
        .clone()
        .max_by_key(|(row, _column, _id)| *row)
        .unwrap()
        .0
        - 1;

    println!(
        "my seat is between: min row = {}, max row = {}",
        min_row, max_row
    );

    let mut possible_ids = (min_row..=max_row)
        .flat_map(|row| (0..=7).map(move |column| Some(row * 8 + column)))
        .collect::<Vec<_>>();

    ids.for_each(|(row, column, id)| {
        if (min_row..=max_row).contains(&row) {
            assert_eq!(Some(id), possible_ids[(row - min_row) * 8 + column]);
            possible_ids[(row - min_row) * 8 + column] = None;
        }
    });

    let seat = possible_ids.into_iter().find_map(|id| id).unwrap();

    println!("my seat is located at: {}", seat)
}
