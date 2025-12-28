// BFS. Can't be a DFS since then we can take too many bad steps and stumble upon a state we could
// reach easier starting from a different button.
fn one(input: Vec<String>) {
    let now = std::time::Instant::now();
    let mut sum = 0;
    let input = parse(&input);

    let mut queue = std::collections::VecDeque::new();
    let mut visited = std::collections::HashSet::new();

    for (target_lights, buttons, _) in &input {
        let state = vec![false; target_lights.len()];
        let mut fewest_steps = usize::MAX;

        // BFS where we terminate early if we've found a quicker path.
        for button in buttons {
            queue.push_back((button, state.clone(), 0));
        }

        // BFS = Pop_front
        while let Some((button, mut state, mut steps)) = queue.pop_front() {
            if steps >= fewest_steps {
                continue;
            }

            // Apply change by switching buttons.
            for change in button {
                state[*change] = !state[*change]
            }

            steps += 1;
            // If we've reached the target, stop searching.
            if state == *target_lights {
                fewest_steps = steps;
                continue;
            }

            // Add all buttons again
            if visited.insert(state.clone()) {
                for button in buttons {
                    queue.push_back((button, state.clone(), steps));
                }
            }
        }
        sum += fewest_steps;

        queue.clear();
        visited.clear();
    }

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}");
}

// Now the search space is even larger.... Now we need to be smart. A regular BFS doesn't even terminate for the first problem.
fn two(input: Vec<String>) {
    let now = std::time::Instant::now();
    let mut sum = 0;
    let input = parse(&input);

    let mut queue = std::collections::VecDeque::new();
    let mut visited = std::collections::HashSet::new();
    for (idx, (_, buttons, target_joltage)) in input.iter().enumerate() {
        let state = vec![0_usize; target_joltage.len()];
        let mut fewest_steps = usize::MAX;

        // BFS where we terminate early if we've found a quicker path.
        for button in buttons {
            queue.push_back((button, state.clone(), 0));
        }

        // BFS = Pop_front
        while let Some((button, mut state, mut steps)) = queue.pop_front() {
            if steps >= fewest_steps {
                continue;
            }

            // If any button value is above the target then kill the branch.
            if state.iter().zip(target_joltage.iter()).any(|(a, b)| a > b) {
                continue;
            }

            // Apply change by switching buttons.
            for change in button {
                state[*change] += 1
            }

            steps += 1;
            // If we've reached the target, stop searching.
            if state == *target_joltage {
                fewest_steps = steps;
                continue;
            }

            // Add all buttons again
            if visited.insert(state.clone()) {
                for button in buttons {
                    queue.push_back((button, state.clone(), steps));
                }
            }
        }
        sum += fewest_steps;

        queue.clear();
        visited.clear();
        println!("Solved: {} of {}", idx, input.len());
    }

    let elapsed = now.elapsed();
    println!("Two: {sum} | Elapsed: {elapsed:?}");
}

// Not the most efficientbut got enough.
fn parse(input: &[String]) -> Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<usize>)> {
    input
        .iter()
        .map(|row| {
            let mut iter = row.split_ascii_whitespace();

            let lights = iter
                .next()
                .unwrap()
                .trim_matches(['[', ']'])
                .chars()
                .map(|light| match light {
                    '#' => true,
                    '.' => false,
                    unknown => unreachable!("Got: {unknown}"),
                })
                .collect::<Vec<_>>();

            // Skip first and last.
            let joltage: Vec<usize> = iter
                .next_back()
                .unwrap()
                .trim_matches(['{', '}'])
                .split(',')
                .map(|joltage| joltage.parse().unwrap())
                .collect();

            let buttons: Vec<Vec<usize>> = iter
                .map(|button| {
                    button
                        .trim_matches(['(', ')'])
                        .split(",")
                        .map(|light| light.parse().unwrap())
                        .collect()
                })
                .collect();
            (lights, buttons, joltage)
        })
        .collect()
}

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();

    one(input.clone());
    two(input);
}
