use std::io::Read;

#[derive(Clone)]
struct Input {
    range_begin: Vec<usize>,
    range_end: Vec<usize>,
    ids: Vec<usize>,
}

fn one(mut input: Input) {
    let now = std::time::Instant::now();
    let sum = 0;

    input.range_begin.sort_unstable();
    input.range_end.sort_unstable();
    input.ids.sort_unstable();

    let first_range_id = input.range_begin.first().unwrap();
    let last_range_id = input.range_end.last().unwrap();
    let first_id = input.ids.first().unwrap();
    let last_id = input.ids.last().unwrap();

    println!("{first_range_id}\n{last_range_id}");
    println!("{}",);

    let ranges = vec![last_range_id - first_range_id; 0];
    println!("items: {}", ranges.len());

    // for range in input.range_begin {}

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}");
}

fn two(_input: &Input) {
    let now = std::time::Instant::now();
    let sum = 0;

    let elapsed = now.elapsed();
    println!("Two: {sum} | Elapsed: {elapsed:?}");
}

fn parse(input: &String) -> Input {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let (range_begin, range_end) = ranges
        .lines()
        .map(|range| {
            let (begin, end) = range.split_once("-").unwrap();
            (
                begin.parse::<usize>().unwrap(),
                end.parse::<usize>().unwrap(),
            )
        })
        .unzip();
    let ids = ids.lines().map(|id| id.parse().unwrap()).collect();
    Input {
        range_begin,
        range_end,
        ids,
    }
}

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();
    let input = parse(&input);

    one(input.clone());
    two(&input);
}
