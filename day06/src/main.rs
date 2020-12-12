fn main() {
    let contents =
        std::fs::read_to_string("day06/input").expect("Something went wrong reading the file");

    let groups = contents.split("\n\n");

    let questions = "abcdefghijklmnopqrstuvwxyz";

    let counts = groups
        .map(|group_persons| {
            // println!(
            //     "\n{}",
            //     group_persons
            //         .trim()
            //         .split('\n')
            //         .collect::<Vec<_>>()
            //         .join(", ")
            // );
            let mut answer_counts = [0; 26];

            let group_persons = group_persons.trim().split('\n');
            let person_count = group_persons.clone().count();
            group_persons.for_each(|person_answers| {
                person_answers.chars().for_each(|answer| {
                    answer_counts[questions.find(answer).unwrap()] += 1;
                });
            });
            // println!("{}", person_count);
            // println!(
            //     "{}",
            //     answer_counts
            //         .iter()
            //         .map(|n| n.to_string())
            //         .collect::<Vec<_>>()
            //         .join(" ")
            // );
            let count_any = answer_counts.iter().filter(|count| **count > 0).count();
            let count_all = answer_counts
                .iter()
                .filter(|count| **count == person_count)
                .count();
            // println!("count_any = {}", count_any);
            // println!("count_all = {}", count_all);
            (count_any, count_all)
        })
        .fold((0, 0), |(sum_any, sum_all), (any, all)| {
            (sum_any + any, sum_all + all)
        });

    println!(
        "total groups that answered any questions with \"yes\": {}",
        counts.0
    );
    println!(
        "total groups that answered all questions with \"yes\": {}",
        counts.1
    );
}
