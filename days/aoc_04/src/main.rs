type Input = Vec<Vec<u64>>;

mod points;

// Idea:
// 1. Add one layer of padding to the input to make it easy to deal with.
// 2. Iterate over input in chunks of 9x9. If paper roll sum number of paper rolls in square.
// 3. If less than 4 then add.
fn one(input: &Input) {
    let now = std::time::Instant::now();
    let sum: u64 = input
        // Iterate three rows at a time.
        .windows(3)
        .map(|window| {
            // Iterate over 3 elements at a time from each row creating a 3x3 area
            window[0]
                .windows(3)
                .zip(window[1].windows(3))
                .zip(window[2].windows(3))
                .map(|((row_0, row_1), row_2)| {
                    if row_1[1] == 1 {
                        if row_0.iter().sum::<u64>()
                            + row_1.iter().sum::<u64>()
                            + row_2.iter().sum::<u64>()
                            <= 4
                        {
                            1
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        })
        .sum();

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}");
}

fn two(mut input: Input) {
    let now = std::time::Instant::now();
    let mut sum = 0;

    loop {
        let to_remove: Vec<(usize, usize)> = input
            // Iterate three rows at a time.
            .windows(3)
            .enumerate()
            .flat_map(|(row, window)| {
                // Iterate over 3 elements at a time from each row creating a 3x3 area
                window[0]
                    .windows(3)
                    .zip(window[1].windows(3))
                    .zip(window[2].windows(3))
                    .enumerate()
                    .filter_map(|(col, ((row_0, row_1), row_2))| {
                        if row_1[1] == 1 {
                            if row_0.iter().sum::<u64>()
                                + row_1.iter().sum::<u64>()
                                + row_2.iter().sum::<u64>()
                                <= 4
                            {
                                Some((row + 1, col + 1))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        if to_remove.is_empty() {
            break;
        }
        sum += to_remove.len();

        for (row, col) in to_remove {
            input[row][col] = 0;
        }
    }

    let elapsed = now.elapsed();
    println!("Two: {sum} | Elapsed: {elapsed:?}");
}

fn parse(input: &[String]) -> Input {
    let input = input
        .iter()
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    '@' => 1,
                    '.' => 0,
                    _ => unreachable!("Unknown type"),
                })
                .collect()
        })
        .collect();
    aoc_lib::pad_input(input, 0)
}

// Points
// One: 1474 | Elapsed: 2.116646ms
// Two: 0 | Elapsed: 30ns

// Grid
// One: 1474 | Elapsed: 63.509µs
// Two: 8910 | Elapsed: 1.827053ms

// HashSets are slow.
fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();
    println!("Points");
    let input_points = points::parse(&input);
    points::one(&input_points);
    // Part 2 skipped because it is the same but just running multiple times until converging.

    println!("\nGrid");
    let input_2d = parse(&input);
    one(&input_2d);
    two(input_2d);
}
