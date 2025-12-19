type Input = Vec<Vec<u64>>;
// type Input = Vec<Vec<Battery>>;

#[derive(Debug, Clone)]
struct Battery {
    joltage: u64,
    idx: usize,
}

fn one(input: Input) {
    let now = std::time::Instant::now();
    let mut sum = 0;

    for row in input {
        let mut highest = (0, 0);
        for (idx, battery) in row.iter().cloned().enumerate() {
            // Only update the first battery if we also can update the
            // second without indexing out of bounds.
            if battery > highest.0 && idx < (row.len() - 1) {
                highest.0 = battery;
                highest.1 = 0;
            } else if battery > highest.1 {
                highest.1 = battery
            }
        }
        sum += highest.0 * 10 + highest.1
    }

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}");
}

// Solution: Do the most leftmost update possible of the current max number while leaving enough
// spots to fill the remaining numbers with new ones.
fn two(input: Input) {
    let now = std::time::Instant::now();
    let mut sum = 0;
    const NUM_BATTERIES_COMBINED: usize = 12;

    for row in input {
        // Now we need to find the highest 12 batteries... So we need to make the solution from part 1 more generic.
        let mut max = vec![0_u64; NUM_BATTERIES_COMBINED];
        let mut update = None;

        for (idx, battery) in row.iter().enumerate() {
            for (curr_highest_loc, curr_highest) in max.iter().enumerate() {
                // Check number of zeros we need to add if finding a new highest number for this location.
                let zeroes_to_add = NUM_BATTERIES_COMBINED - curr_highest_loc - 1;
                if battery > curr_highest && zeroes_to_add < row.len() - idx {
                    // Schedule an update when we've dropped the reference to the max Vec.
                    // We could either manually iterate, or do it this way.
                    update = Some((curr_highest_loc, battery));
                    // Break to ensure that we don't keep comparing stuff after already having updated a number.
                    break;
                }
            }
            if let Some((curr_highest_loc, battery)) = update {
                max[curr_highest_loc] = *battery;
                // Set all items after loc to zero. We've already made sure that we can fill these numbers
                // zeros to add check.
                for elem in max[curr_highest_loc + 1..].iter_mut() {
                    *elem = 0;
                }
            }
        }
        sum += max.iter().fold(0, |acc, &d| acc * 10 + d);
    }

    let elapsed = now.elapsed();
    println!("Two: {sum} | Elapsed: {elapsed:?}");
}

fn parse(input: &[String]) -> Input {
    input
        .iter()
        .map(|row| {
            row.chars()
                .map(|char| char.to_digit(10).unwrap().try_into().unwrap())
                // .enumerate()
                // .map(|(idx, char)| Battery {
                //     joltage: char.to_digit(10).unwrap().try_into().unwrap(),
                //     idx,
                // })
                .collect()
        })
        .collect()
}

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();
    let input = parse(&input);

    one(input.clone());
    two(input);
}
