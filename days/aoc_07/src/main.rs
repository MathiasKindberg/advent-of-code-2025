use std::vec;

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

// fn recurse(
//     mut beam_cols: Vec<(u64, u64)>, // Should maybe just be (u64 u64?)
//     splitters: std::collections::HashSet<(u64, u64)>,
// ) -> u64 {
//     for (beam_row, beam_col) in beam_cols {
//         // Take a step down, if splitter then SPLIT IT!
//         if splitters.contains(&(beam_row + 1, beam_col)) {
//             println!("SPLIT!");
//         }
//     }
//     0
// }

// Typical graph traversal problem. Recursively collect the number of outflowing valid paths
// at each splitter. Then make it efficient by adding the numbers upward and termitnating early if
// the path is already known.
fn two(input: Vec<String>) {
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

    let total_rows = input.len().try_into().unwrap();

    let splitters: std::collections::HashSet<(u64, u64)> = input
        .into_iter()
        // Remove all rows with only dots.
        .filter(|row| !row.chars().into_iter().all(|char| char == '.'))
        .enumerate()
        .skip(1)
        // Create a flat hashset of all splitters.
        .flat_map(|(row_idx, row)| {
            row.char_indices()
                .filter_map(|(col_idx, elem)| match elem {
                    '^' => Some((
                        u64::try_from(row_idx).unwrap(),
                        u64::try_from(col_idx).unwrap(),
                    )),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect();

    // This is correct but since we're double checking so many paths it takes "forever" to run.
    let mut to_visit = vec![(0_u64, source)];
    while let Some((beam_row, beam_col)) = to_visit.pop() {
        let beam_row = beam_row + 1;
        if beam_row == total_rows {
            sum += 1;
            continue;
        }

        if splitters.contains(&(beam_row, beam_col)) {
            // Split!
            to_visit.push((beam_row, beam_col - 1));
            to_visit.push((beam_row, beam_col + 1))
        } else {
            // Go down!
            to_visit.push((beam_row, beam_col));
        }
    }

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
