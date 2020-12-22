use rayon::prelude::*;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

#[derive(Debug)]
enum Cursor {
    A,
    B,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position {
    column: usize,
    row: usize,
}

/// Run with:
/// ```no_run
/// cargo run --package day11 --release
/// cargo run --package day11 --release -Z package-features --features print_occupancy
/// cargo run --package day11 --release -Z package-features --features print_visible
/// cargo run --package day11 --release -Z package-features --features print_occupancy --features print_visible
/// ```
fn main() {
    let time_start = Instant::now();

    let contents =
        std::fs::read_to_string("day11/input").expect("Something went wrong reading the file");
    #[cfg(any(feature = "print_occupancy", feature = "print_visible"))]
    const COLUMNS: usize = 90;

    let time_file_read = Instant::now();
    // let contents = r#"
    // L.LL.LL.LL
    // LLLLLLL.LL
    // L.L.L..L..
    // LLLL.LL.LL
    // L.LL.LL.LL
    // L.LLLLL.LL
    // ..L.L.....
    // LLLLLLLLLL
    // L.LLLLLL.L
    // L.LLLLL.LL
    // "#
    // .to_string();
    // #[cfg(any(feature = "print_occupancy", feature = "print_visible"))]
    // const COLUMNS: usize = 10;

    let (positions, seats): (Vec<_>, Vec<_>) = contents
        .split_whitespace()
        .enumerate()
        .flat_map(|(row, columns)| {
            columns.chars().enumerate().map(move |(column, seat)| {
                (
                    Position { column, row },
                    match seat {
                        'L' => Seat::Empty,
                        '#' => Seat::Occupied,
                        '.' => Seat::Floor,
                        _ => unreachable!(),
                    },
                )
            })
        })
        .unzip();

    let time_mapped = Instant::now();

    // --------------------------------------------

    let tolerance = 4;

    let mut seats_a = seats.clone();
    #[cfg(feature = "print_occupancy")]
    print_occupancy(&positions, &seats_a, COLUMNS, tolerance, adjacent_occupied);
    #[cfg(feature = "print_visible")]
    print_visible(&positions, &seats_a, COLUMNS, tolerance, adjacent_occupied);

    let mut seats_b = next(&positions, &seats_a, tolerance, adjacent_occupied);
    #[cfg(feature = "print_occupancy")]
    print_occupancy(&positions, &seats_b, COLUMNS, tolerance, adjacent_occupied);
    #[cfg(feature = "print_visible")]
    print_visible(&positions, &seats_b, COLUMNS, tolerance, adjacent_occupied);

    let mut cursor = Cursor::B;

    while seats_a != seats_b {
        // if cursor = B, replace seats_a, calculated from seats_b
        // if cursor = A, replace seats_b, calculated from seats_a
        let (current_seats, next_seats) = match cursor {
            Cursor::A => (&seats_a, &mut seats_b),
            Cursor::B => (&seats_b, &mut seats_a),
        };

        let new_seats = next(&positions, current_seats, tolerance, adjacent_occupied);
        #[cfg(feature = "print_occupancy")]
        print_occupancy(
            &positions,
            &new_seats,
            COLUMNS,
            tolerance,
            adjacent_occupied,
        );
        #[cfg(feature = "print_visible")]
        print_visible(
            &positions,
            &new_seats,
            COLUMNS,
            tolerance,
            adjacent_occupied,
        );
        *next_seats = new_seats;

        // swap the cursor
        cursor = match cursor {
            Cursor::A => Cursor::B,
            Cursor::B => Cursor::A,
        }
    }

    // seats_a and seats_b are the same now so it doesn't matter which
    let final_occupied_seats = seats_a
        .par_iter()
        .filter(|seat| **seat == Seat::Occupied)
        .count();

    println!("final occupied seats (rule 1): {}\n", final_occupied_seats);

    let time_rule1_finished = Instant::now();

    // --------------------------------------------

    let tolerance = 5;

    let mut seats_a = seats.clone();
    #[cfg(feature = "print_occupancy")]
    print_occupancy(&positions, &seats_a, COLUMNS, tolerance, los_occupied);
    #[cfg(feature = "print_visible")]
    print_visible(&positions, &seats_a, COLUMNS, tolerance, los_occupied);

    let mut seats_b = next(&positions, &seats_a, tolerance, los_occupied);
    #[cfg(feature = "print_occupancy")]
    print_occupancy(&positions, &seats_b, COLUMNS, tolerance, los_occupied);
    #[cfg(feature = "print_visible")]
    print_visible(&positions, &seats_b, COLUMNS, tolerance, los_occupied);

    let mut cursor = Cursor::B;

    while seats_a != seats_b {
        // if cursor = B, replace seats_a, calculated from seats_b
        // if cursor = A, replace seats_b, calculated from seats_a
        let (current_seats, next_seats) = match cursor {
            Cursor::A => (&seats_a, &mut seats_b),
            Cursor::B => (&seats_b, &mut seats_a),
        };

        let new_seats = next(&positions, current_seats, tolerance, los_occupied);
        #[cfg(feature = "print_occupancy")]
        print_occupancy(&positions, &new_seats, COLUMNS, tolerance, los_occupied);
        #[cfg(feature = "print_visible")]
        print_visible(&positions, &new_seats, COLUMNS, tolerance, los_occupied);
        *next_seats = new_seats;

        // swap the cursor
        cursor = match cursor {
            Cursor::A => Cursor::B,
            Cursor::B => Cursor::A,
        }
    }

    // seats_a and seats_b are the same now so it doesn't matter which
    let final_occupied_seats = seats_a
        .par_iter()
        .filter(|seat| **seat == Seat::Occupied)
        .count();

    println!("final occupied seats (rule 2): {}\n", final_occupied_seats);

    let time_rule2_finished = Instant::now();

    drop(seats);

    println!("time to read file: {:?}", time_file_read - time_start);
    println!("time to map data: {:?}", time_mapped - time_file_read);
    println!(
        "time to complete rule 1: {:?}",
        time_rule1_finished - time_mapped
    );
    println!(
        "time to complete rule 2: {:?}",
        time_rule2_finished - time_rule1_finished
    );
    println!("total time: {:?}", time_rule2_finished - time_start);
}

fn next(
    positions: &[Position],
    current_seats: &[Seat],
    tolerance: usize,
    visible_seats: fn(&[Position], &[Seat], usize) -> usize,
) -> Vec<Seat> {
    current_seats
        .par_iter()
        .enumerate()
        .map(|(current_, seat)| match *seat {
            Seat::Empty => {
                if visible_seats(positions, current_seats, current_) == 0 {
                    Seat::Occupied
                } else {
                    Seat::Empty
                }
            }
            Seat::Occupied => {
                if visible_seats(positions, current_seats, current_) >= tolerance {
                    Seat::Empty
                } else {
                    Seat::Occupied
                }
            }
            Seat::Floor => Seat::Floor,
        })
        .collect::<Vec<_>>()
}

fn adjacent_occupied(positions: &[Position], current_seats: &[Seat], current_: usize) -> usize {
    let Position { column, row } = positions[current_];
    positions
        .par_iter()
        .enumerate()
        // not this position
        .filter(|(_, position)| !(position.column == column && position.row == row))
        // adjacent
        .filter(|(_, position)| {
            // x +- 1
            (position.column + 1 >= column && position.column <= column + 1)
            // y +- 1
            && (position.row + 1 >= row && position.row <= row + 1)
        })
        // occupied
        .filter(|(entity, _)| current_seats[*entity] == Seat::Occupied)
        .count()
}

fn los_occupied(positions: &[Position], current_seats: &[Seat], current_: usize) -> usize {
    let Position { column, row } = positions[current_];
    let this = positions[current_];
    let iterator = positions
        .par_iter()
        .enumerate()
        // not this position
        .filter(|(_, position)| !(position.column == column && position.row == row))
        // eight directions from this position;
        .filter_map(|(entity, other)| Direction::compare(&this, &other).map(|dir| (entity, dir)))
        // ignore floor (only look at occupied or empty)
        .filter(|(entity, _)| current_seats[*entity] != Seat::Floor);

    let up = iterator
        .clone()
        .filter(|(_, dir)| matches!(dir, Direction::Up(_)))
        .min_by_key(|(_, dir)| {
            if let Direction::Up(distance) = dir {
                *distance
            } else {
                std::usize::MAX
            }
        })
        .map_or(false, |(entity, _)| current_seats[entity] == Seat::Occupied);

    let up_right = iterator
        .clone()
        .filter(|(_, dir)| matches!(dir, Direction::UpRight(_)))
        .min_by_key(|(_, dir)| {
            if let Direction::UpRight(distance) = dir {
                *distance
            } else {
                std::usize::MAX
            }
        })
        .map_or(false, |(entity, _)| current_seats[entity] == Seat::Occupied);

    let right = iterator
        .clone()
        .filter(|(_, dir)| matches!(dir, Direction::Right(_)))
        .min_by_key(|(_, dir)| {
            if let Direction::Right(distance) = dir {
                *distance
            } else {
                std::usize::MAX
            }
        })
        .map_or(false, |(entity, _)| current_seats[entity] == Seat::Occupied);

    let down_right = iterator
        .clone()
        .filter(|(_, dir)| matches!(dir, Direction::DownRight(_)))
        .min_by_key(|(_, dir)| {
            if let Direction::DownRight(distance) = dir {
                *distance
            } else {
                std::usize::MAX
            }
        })
        .map_or(false, |(entity, _)| current_seats[entity] == Seat::Occupied);

    let down = iterator
        .clone()
        .filter(|(_, dir)| matches!(dir, Direction::Down(_)))
        .min_by_key(|(_, dir)| {
            if let Direction::Down(distance) = dir {
                *distance
            } else {
                std::usize::MAX
            }
        })
        .map_or(false, |(entity, _)| current_seats[entity] == Seat::Occupied);

    let down_left = iterator
        .clone()
        .filter(|(_, dir)| matches!(dir, Direction::DownLeft(_)))
        .min_by_key(|(_, dir)| {
            if let Direction::DownLeft(distance) = dir {
                *distance
            } else {
                std::usize::MAX
            }
        })
        .map_or(false, |(entity, _)| current_seats[entity] == Seat::Occupied);

    let left = iterator
        .clone()
        .filter(|(_, dir)| matches!(dir, Direction::Left(_)))
        .min_by_key(|(_, dir)| {
            if let Direction::Left(distance) = dir {
                *distance
            } else {
                std::usize::MAX
            }
        })
        .map_or(false, |(entity, _)| current_seats[entity] == Seat::Occupied);

    let up_left = iterator
        .clone()
        .filter(|(_, dir)| matches!(dir, Direction::UpLeft(_)))
        .min_by_key(|(_, dir)| {
            if let Direction::UpLeft(distance) = dir {
                *distance
            } else {
                std::usize::MAX
            }
        })
        .map_or(false, |(entity, _)| current_seats[entity] == Seat::Occupied);

    [
        up, up_right, right, down_right, down, down_left, left, up_left,
    ]
    .par_iter()
    .filter(|d| **d)
    .count()
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up(usize),
    UpRight(usize),
    Right(usize),
    DownRight(usize),
    Down(usize),
    DownLeft(usize),
    Left(usize),
    UpLeft(usize),
}

impl Direction {
    fn compare(this: &Position, other: &Position) -> Option<Self> {
        use Direction::*;
        if other.column == this.column && other.row < this.row {
            Some(Up(this.row - other.row))
        } else if other.column == this.column && other.row > this.row {
            Some(Down(other.row - this.row))
        } else if other.row == this.row && other.column > this.column {
            Some(Right(other.column - this.column))
        } else if other.row == this.row && other.column < this.column {
            Some(Left(this.column - other.column))
        // diagonal top-left to bottom-right: y1 = x1 + c && y2 = x2 + c => y1 - x1 = y2 - x2 => (y1 + x2 = y2 + x1)
        } else if this.row + other.column == other.row + this.column && other.row < this.row {
            Some(UpLeft(this.row - other.row))
        } else if this.row + other.column == other.row + this.column && other.row > this.row {
            Some(DownRight(other.row - this.row))
        // diagonal bottom-left to top-right: y1 = -x1 + c && y2 = -x2 + c => (y1 + x1 = y2 + x2)
        } else if this.row + this.column == other.row + other.column && other.row < this.row {
            Some(UpRight(this.row - other.row))
        } else if this.row + this.column == other.row + other.column && other.row > this.row {
            Some(DownLeft(other.row - this.row))
        } else {
            None
        }
    }
}

#[test]
fn test_compare() {
    // --------
    // --------
    // -----x--
    // --------
    // --------
    // --------
    // --------
    // --------
    let this = Position { column: 5, row: 2 };

    // --------
    // --------
    // -----*--
    // --------
    // --------
    // --------
    // --------
    // --------
    let mut other = Position { column: 5, row: 2 };

    assert_eq!(Direction::compare(&this, &other), None);

    // --------
    // --------
    // -----x-*
    // --------
    // --------
    // --------
    // --------
    // --------
    other.column = 7;

    assert_eq!(Direction::compare(&this, &other), Some(Direction::Right(2)));

    // --------
    // --------
    // --*--x--
    // --------
    // --------
    // --------
    // --------
    // --------
    other.column = 2;

    assert_eq!(Direction::compare(&this, &other), Some(Direction::Left(3)));

    // --------
    // --*-----
    // -----x--
    // --------
    // --------
    // --------
    // --------
    // --------
    other.row = 1;

    assert_eq!(Direction::compare(&this, &other), None);

    // --------
    // --------
    // -----x--
    // --------
    // --------
    // --*-----
    // --------
    // --------
    other.row = 5;

    assert_eq!(
        Direction::compare(&this, &other),
        Some(Direction::DownLeft(3))
    );

    // --------
    // --------
    // -----x--
    // --------
    // --------
    // ---*----
    // --------
    // --------
    other.column = 3;

    assert_eq!(Direction::compare(&this, &other), None);

    // --------
    // --------
    // -----x--
    // --------
    // --------
    // -----*--
    // --------
    // --------
    other.column = 5;

    assert_eq!(Direction::compare(&this, &other), Some(Direction::Down(3)));

    // --------
    // --------
    // -----x--
    // --------
    // --------
    // -------*
    // --------
    // --------
    other.column = 7;

    assert_eq!(Direction::compare(&this, &other), None);

    // --------
    // --------
    // -----x--
    // --------
    // -------*
    // --------
    // --------
    // --------
    other.row = 4;

    assert_eq!(
        Direction::compare(&this, &other),
        Some(Direction::DownRight(2))
    );

    // --------
    // -------*
    // -----x--
    // --------
    // --------
    // --------
    // --------
    // --------
    other.row = 1;

    assert_eq!(Direction::compare(&this, &other), None);

    // --------
    // ------*-
    // -----x--
    // --------
    // --------
    // --------
    // --------
    // --------
    other.column = 6;

    assert_eq!(
        Direction::compare(&this, &other),
        Some(Direction::UpRight(1))
    );

    // --------
    // -----*--
    // -----x--
    // --------
    // --------
    // --------
    // --------
    // --------
    other.column = 5;

    assert_eq!(Direction::compare(&this, &other), Some(Direction::Up(1)));

    // --------
    // ----*---
    // -----x--
    // --------
    // --------
    // --------
    // --------
    // --------
    other.column = 4;

    assert_eq!(
        Direction::compare(&this, &other),
        Some(Direction::UpLeft(1))
    );

    // --------
    // ---*----
    // -----x--
    // --------
    // --------
    // --------
    // --------
    // --------
    other.column = 3;

    assert_eq!(Direction::compare(&this, &other), None);

    // ---*----
    // --------
    // -----x--
    // --------
    // --------
    // --------
    // --------
    // --------
    other.row = 0;

    assert_eq!(
        Direction::compare(&this, &other),
        Some(Direction::UpLeft(2))
    );
}

#[cfg(feature = "print_occupancy")]
fn print_occupancy(
    positions: &[Position],
    seats: &[Seat],
    columns: usize,
    tolerance: usize,
    visible_seats: fn(&[Position], &[Seat], usize) -> usize,
) {
    let mut s = String::with_capacity(seats.len());
    seats
        .par_iter()
        .enumerate()
        .collect::<Vec<_>>()
        .chunks_exact(columns)
        .for_each(|chunk| {
            chunk.par_iter().for_each(|(current_, seat)| match *seat {
                Seat::Empty => {
                    let neighbours = visible_seats(&positions, seats, *current_);
                    let to_occupy = neighbours == 0;
                    if to_occupy {
                        s.push_str("\x1b[32m");
                    }
                    s.push('L');
                    if to_occupy {
                        s.push_str("\x1b[0m");
                    }
                }
                Seat::Occupied => {
                    let neighbours = visible_seats(&positions, seats, *current_);
                    let to_empty = neighbours >= tolerance;
                    if to_empty {
                        s.push_str("\x1b[31m");
                    }
                    s.push('#');
                    if to_empty {
                        s.push_str("\x1b[0m");
                    }
                }
                Seat::Floor => {
                    s.push_str("\x1b[38;5;236m");
                    s.push('.');
                    s.push_str("\x1b[0m");
                }
            });
            s.push('\n');
        });
    println!("{}", s);
}

#[cfg(feature = "print_visible")]
fn print_visible(
    positions: &[Position],
    seats: &[Seat],
    columns: usize,
    tolerance: usize,
    visible_seats: fn(&[Position], &[Seat], usize) -> usize,
) {
    let mut s = String::with_capacity(seats.len());
    seats
        .par_iter()
        .enumerate()
        .collect::<Vec<_>>()
        .chunks_exact(columns)
        .for_each(|chunk| {
            chunk.par_iter().for_each(|(current_, seat)| match *seat {
                Seat::Empty => {
                    let neighbours = visible_seats(&positions, seats, *current_);
                    let to_occupy = neighbours == 0;
                    if to_occupy {
                        s.push_str("\x1b[32m");
                    }
                    s.push_str(neighbours.to_string().as_str());
                    if to_occupy {
                        s.push_str("\x1b[0m");
                    }
                }
                Seat::Occupied => {
                    let neighbours = visible_seats(&positions, seats, *current_);
                    let to_empty = neighbours >= tolerance;
                    if to_empty {
                        s.push_str("\x1b[31m");
                    }
                    s.push_str(neighbours.to_string().as_str());
                    if to_empty {
                        s.push_str("\x1b[0m");
                    }
                }
                Seat::Floor => {
                    s.push_str("\x1b[38;5;236m");
                    s.push('.');
                    s.push_str("\x1b[0m");
                }
            });
            s.push('\n');
        });
    println!("{}", s);
}
