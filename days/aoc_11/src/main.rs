const END: &str = "out";
const MUST_VISIT: [&str; 2] = ["dac", "fft"];

// Extremely simple brute force DFS. For example no smartness keeping track of
// which paths we've taken to reduce the search space.
fn dfs_part_one<'a>(
    adjacency_list: &std::collections::HashMap<&'a str, Vec<&'a str>>,
    queue: &mut Vec<&'a str>,
) -> u64 {
    let edges = adjacency_list.get(queue.pop().unwrap()).unwrap();
    let mut sum = 0;
    for next_vertice in edges {
        // If we found an end using this path, return 1.
        if *next_vertice == END {
            return 1;
        } else {
            // Else sum up all ends found by taking the next step.
            queue.push(*next_vertice);
            sum += dfs_part_one(adjacency_list, queue);
        }
    }
    sum
}

// Idea: DFS with a list of visited nodes.
fn one(input: Vec<String>) {
    let now = std::time::Instant::now();
    let adjacency_list: std::collections::HashMap<_, _> = input
        .iter()
        .map(|row| row.split_once(": ").unwrap())
        .map(|(src, dest)| (src, dest.split_ascii_whitespace().collect::<Vec<_>>()))
        .collect();

    let sum = dfs_part_one(&adjacency_list, &mut vec!["you"]);

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}");
}

// I bet there's no cycles and that we need to manage a magnitudes larger search space.
//
// 1. Brute force for like a minute = no cycle so shouldnt exist any.
//
// Time to add memoization....
fn dfs_part_two<'a>(
    adjacency_list: &std::collections::HashMap<&'a str, Vec<&'a str>>,
    queue: &mut Vec<&'a str>,
    // path: Vec<&'a str>,
    // Need to sort the memoization key.
    visited_required: Vec<&'a str>,
    memoization: &mut std::collections::HashMap<(&'a str, Vec<&'a str>), u64>,
) -> u64 {
    let current = queue.pop().unwrap();

    // Check if we've already seen this grouping of current vertice and visited requirements.
    if let Some(&cached) = memoization.get(&(current, visited_required.clone())) {
        return cached;
    }

    let edges = adjacency_list.get(current).unwrap();
    let mut sum = 0;

    for next_vertice in edges {
        if *next_vertice == END {
            if visited_required.len() == MUST_VISIT.len() {
                sum += 1
            }
        } else {
            // Else sum up all ends found by taking the next step.
            queue.push(*next_vertice);

            let mut visited_required: Vec<&str> = visited_required.clone();

            if MUST_VISIT.contains(next_vertice) {
                if !visited_required.contains(next_vertice) {
                    visited_required.push(next_vertice);
                    // Since we're using a Vec we need to ensure that the order is always the
                    // same. A HashSet or btreeset would be neater but contain unnecessary overhead
                    // for a Vec of size 2.
                    visited_required.sort_unstable();
                }
            }

            // Copy the current path every time we split into the outbound edges.
            sum += dfs_part_two(adjacency_list, queue, visited_required, memoization);
        }
    }
    memoization.insert((current, visited_required), sum);
    sum
}

fn two(input: Vec<String>) {
    let now = std::time::Instant::now();
    let adjacency_list: std::collections::HashMap<_, _> = input
        .iter()
        .map(|row| row.split_once(": ").unwrap())
        .map(|(src, dest)| (src, dest.split_ascii_whitespace().collect::<Vec<_>>()))
        .collect();

    let sum = dfs_part_two(
        &adjacency_list,
        &mut vec!["svr"],
        Vec::new(),
        // std::collections::HashSet::new(),
        &mut std::collections::HashMap::new(),
    );

    let elapsed = now.elapsed();
    println!("Two: {sum} | Elapsed: {elapsed:?}");
}

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();

    one(input.clone());
    two(input);
}
