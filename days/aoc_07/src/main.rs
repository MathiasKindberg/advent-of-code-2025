type Input = Vec<String>;

// Count number of splits if it has a splitter
// above it and -1 or +1 in columns.
fn one(input: Vec<String>) {
    let now = std::time::Instant::now();
    let mut sum = 0;

    let source = u64::try_from(
        input[0]
            .char_indices()
            .find(|(_, elem)| elem == &'S')
            .unwrap()
            .0,
    )
    .unwrap();

    let mut beam_cols: std::collections::HashSet<_> = vec![source].into_iter().collect();
    let mut new_cols = std::collections::HashSet::new();

    let mut input: Vec<_> = input
        .into_iter()
        .skip(1)
        .filter_map(|row| {
            let row = row
                .char_indices()
                .filter_map(|(col, elem)| match elem {
                    '^' => Some(u64::try_from(col).unwrap()),
                    _ => None,
                })
                .collect::<Vec<_>>();
            if row.is_empty() { None } else { Some(row) }
        })
        .collect();
    // We want to iterate top down
    input.reverse();

    while let Some(row) = input.pop() {
        for splitter in row {
            if beam_cols.contains(&splitter) {
                // Insert the new split beamss
                new_cols.insert(splitter - 1);
                new_cols.insert(splitter + 1);

                // Remove the terminated beam
                beam_cols.remove(&splitter);
                sum += 1;
            }
        }

        // Extend the beams which did not meet a splitter.
        new_cols.extend(beam_cols.drain());

        // Swap memory around.
        std::mem::swap(&mut beam_cols, &mut new_cols)
    }

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}");
}

// Typical graph finding problem. Recursively collect the number of outflowing valid paths
// at each splitter. Then make it efficient by adding the numbers upward and termitnating early if
// the path is already known.
fn two(_input: &Input) {
    let now = std::time::Instant::now();
    let sum = 0;

    let elapsed = now.elapsed();
    println!("Two: {sum} | Elapsed: {elapsed:?}");
}

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();
    one(input.clone());
    two(&input);
}
