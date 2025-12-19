type Input = Vec<(u64, u64)>;

fn one(input: &Input) {
    let now = std::time::Instant::now();
    let mut sum = 0;

    for (low, high) in input {
        for id in (*low)..=(*high) {
            let digits = id.checked_ilog10().unwrap_or(0) + 1;
            // Can only have even number of digits to split into two matching parts.
            if !digits % 2 == 0 {
                continue;
            }
            let base = 10_u64.pow(digits / 2);
            let first_part = id / base;
            let second_part = id % base;

            if first_part == second_part {
                sum += id
            }
        }
    }

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}");
}

/// start_digit = starting position (0-indexed from the left/most significant digit)
fn get_part(number: u64, start_digit: u32, steps: u32) -> Option<u64> {
    let total_digits = number.checked_ilog10().unwrap_or(0) + 1;

    // Check if extraction is valid
    if start_digit + steps > total_digits {
        return None;
    }

    // Remove digits to the right of our target slice
    let digits_to_remove_right = total_digits - start_digit - steps;
    let trimmed_right = number / 10_u64.pow(digits_to_remove_right);

    // Keep only n digits
    Some(trimmed_right % 10_u64.pow(steps))
}

// Brute force, but integer math and smart about exclusions.
fn two(input: &Input) {
    let now = std::time::Instant::now();
    let mut sum = 0;

    let mut deduplicate_repeating_parts = Vec::new();

    for (low, high) in input.iter() {
        for id in (*low)..=(*high) {
            let digits = id.checked_ilog10().unwrap_or(0) + 1;
            // Cant repeat if less than 2 digits.
            if digits < 2 {
                continue;
            }

            // Can't have repititions if they are longer than half
            for repeating_digits in 1..=(digits / 2) {
                // Can't have repititions if we can't cleanly divide.
                if digits % repeating_digits != 0 {
                    continue;
                }

                let repeating_part = get_part(id, 0, repeating_digits).unwrap();

                // Now step through the number checking if the repeating part matches
                let mut start_digit = repeating_digits;
                let mut matches = true;
                while start_digit < digits {
                    if repeating_part != get_part(id, start_digit, repeating_digits).unwrap() {
                        matches = false;
                        break;
                    }
                    start_digit += repeating_digits
                }

                // Need to de-duplicate numbers like: 222222 which matches on 1, 2 and 3 repeating digits.
                if matches && !deduplicate_repeating_parts.contains(&id) {
                    deduplicate_repeating_parts.push(id);
                    sum += id;
                }
            }

            // Clear the deduplication Vec.
            deduplicate_repeating_parts.clear();
        }
    }

    let elapsed = now.elapsed();
    println!("Two: {sum} | Elapsed: {elapsed:?}");
}

fn parse(input: String) -> Input {
    input
        .split(",")
        .into_iter()
        .map(|input| input.split_once("-").expect("Valid input"))
        .map(|(low, high)| (low.parse().unwrap(), high.parse().unwrap()))
        .collect()
}

fn main() {
    let stdin = std::io::stdin();
    let mut input: String = String::new();
    stdin.read_line(&mut input).unwrap();

    let input = parse(input);
    // println!("{input:?}");

    one(&input);
    two(&input);
}

#[cfg(test)]
mod tests {
    use crate::get_part;

    #[test]
    fn test_get_part() {
        struct Test {
            number: u64,
            start_digit: u32,
            steps: u32,
            correct: u64,
        }

        let test_table = vec![
            Test {
                number: 123456789,
                start_digit: 2,
                steps: 1,
                correct: 3,
            },
            Test {
                number: 12,
                start_digit: 0,
                steps: 1,
                correct: 1,
            },
            Test {
                number: 12,
                start_digit: 1,
                steps: 1,
                correct: 2,
            },
            Test {
                number: 123,
                start_digit: 0,
                steps: 2,
                correct: 12,
            },
            Test {
                number: 123,
                start_digit: 1,
                steps: 2,
                correct: 23,
            },
            Test {
                number: 123,
                start_digit: 0,
                steps: 3,
                correct: 123,
            },
            Test {
                number: 123,
                start_digit: 1,
                steps: 1,
                correct: 2,
            },
        ];

        for test in test_table {
            assert_eq!(
                get_part(test.number, test.start_digit, test.steps).unwrap(),
                test.correct,
                "For {} starting at digit {} and ending at {}",
                test.number,
                test.start_digit,
                test.steps
            )
        }
    }
}
