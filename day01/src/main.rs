fn main() {
    let contents =
        std::fs::read_to_string("day01/input").expect("Something went wrong reading the file");
    let numbers = contents
        .lines()
        .map(|line| line.parse::<u32>().expect("Not a number"));

    numbers.clone().enumerate().for_each(|(index, number)| {
        numbers.clone().skip(index + 1).for_each(|other_number| {
            if number + other_number == 2020 {
                println!(
                    "a = {}, b = {}, a + b = {}, a * b = {}",
                    number,
                    other_number,
                    number + other_number,
                    number * other_number
                );
            }
        });
    });

    numbers.clone().enumerate().for_each(|(index, number)| {
        let other_numbers = numbers.clone().skip(index + 1);
        other_numbers
            .clone()
            .enumerate()
            .for_each(|(other_index, other_number)| {
                other_numbers
                    .clone()
                    .skip(other_index + 1)
                    .for_each(|third_number| {
                        if number + other_number + third_number == 2020 {
                            println!(
                                "a = {}, b = {}, c = {}, a + b + c = {}, a * b * c = {}",
                                number,
                                other_number,
                                third_number,
                                number + other_number + third_number,
                                number * other_number * third_number
                            );
                        }
                    });
            });
    });
}
