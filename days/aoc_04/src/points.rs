type Input = std::collections::HashSet<(isize, isize)>;

const ADJACENCY_LIST: [(isize, isize); 8] = [
    // Above
    (-1, -1),
    (-1, 0),
    (-1, 1),
    // Same row
    (0, -1),
    (0, 1),
    // Below
    (1, -1),
    (1, 0),
    (1, 1),
];

// Idea:
// 1. Add one layer of padding to the input to make it easy to deal with.
// 2. Iterate over input in chunks of 9x9. If paper roll sum number of paper rolls in square.
// 3. If less than 4 then add.
pub fn one(input: &Input) {
    let now = std::time::Instant::now();

    let sum = input
        .iter()
        .filter(|(row, col)| {
            ADJACENCY_LIST
                .iter()
                .filter(|(adj_row, adj_col)| input.contains(&(row + adj_row, col + adj_col)))
                .count()
                < 4
        })
        .count();

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}");
}

pub fn parse(input: &[String]) -> Input {
    input
        .iter()
        .enumerate()
        .flat_map(|(row, elems)| {
            elems
                .chars()
                .enumerate()
                .filter_map(|(col, c)| match c {
                    '@' => Some((row.try_into().unwrap(), col.try_into().unwrap())),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
