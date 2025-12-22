use std::{collections::VecDeque, io::Read};

#[derive(Debug)]
enum Op {
    Begin(u64),
    End(u64),
}

impl Op {
    pub fn id(&self) -> u64 {
        match &self {
            Op::Begin(id) => *id,
            Op::End(id) => *id,
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    ranges: Vec<(u64, u64)>,
    ids: Vec<u64>,
}

fn get_first_open_range(ranges: &mut VecDeque<(u64, i32)>) -> Option<(u64, u64)> {
    let (n_begin, (begin_id, _)) = ranges
        .iter()
        .cloned()
        .enumerate()
        .find(|(_, (_, state))| state >= &1)?;

    let (n_end, (end_id, _)) = ranges
        .iter()
        .cloned()
        .skip(n_begin)
        .enumerate()
        .find(|(_, (_, state))| state == &0)?;

    ranges.drain(..(n_begin + n_end));

    Some((begin_id, end_id))
}

fn one(mut input: Input) {
    let now = std::time::Instant::now();
    let mut sum = 0;
    input.ids.sort_unstable();

    let mut ranges: Vec<Op> = input
        .ranges
        .into_iter()
        .flat_map(|(a, b)| [Op::Begin(a), Op::End(b)])
        .collect();

    ranges.sort_unstable_by(|a, b| {
        // If equal we always order Begin before End to prevent state counter going below zero.
        a.id().cmp(&b.id()).then_with(|| match (a, b) {
            (Op::Begin(_), Op::End(_)) => std::cmp::Ordering::Less,
            (Op::End(_), Op::Begin(_)) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        })
    });

    let mut open_ranges: VecDeque<_> = ranges
        .into_iter()
        .scan(0, |state, elem| {
            match &elem {
                Op::Begin(_) => *state += 1,
                Op::End(_) => *state -= 1,
            };
            // State should never be below zero, that means some ordering is incorrect.
            assert!(*state >= 0, "State: {state} {elem:?}");
            Some((elem.id(), *state))
        })
        .collect();

    let (mut begin_id, mut end_id) = get_first_open_range(&mut open_ranges).unwrap();
    for id in input.ids {
        // If current id is past the end ID step forward to next range.
        if id > end_id {
            if let Some((new_begin_id, new_end_id)) = get_first_open_range(&mut open_ranges) {
                begin_id = new_begin_id;
                end_id = new_end_id;
            } else {
                break;
            }
        }
        // If in range add one to sum
        if id >= begin_id && id <= end_id {
            sum += 1;
        }
    }

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}");
}

fn two(input: Input) {
    let now = std::time::Instant::now();
    let mut sum = 0;

    let mut ranges: Vec<Op> = input
        .ranges
        .into_iter()
        .flat_map(|(a, b)| [Op::Begin(a), Op::End(b)])
        .collect();

    ranges.sort_unstable_by(|a, b| {
        // If equal we always order Begin before End to prevent state counter going below zero.
        a.id().cmp(&b.id()).then_with(|| match (a, b) {
            (Op::Begin(_), Op::End(_)) => std::cmp::Ordering::Less,
            (Op::End(_), Op::Begin(_)) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        })
    });

    let mut open_ranges: VecDeque<_> = ranges
        .into_iter()
        .scan(0, |state, elem| {
            match &elem {
                Op::Begin(_) => *state += 1,
                Op::End(_) => *state -= 1,
            };
            // State should never be below zero, that means some ordering is incorrect.
            assert!(*state >= 0, "State: {state} {elem:?}");
            Some((elem.id(), *state))
        })
        .collect();

    while let Some((begin_id, end_id)) = get_first_open_range(&mut open_ranges) {
        // +1 since we are inclusive on both sides.
        sum += end_id - begin_id + 1;
    }

    let elapsed = now.elapsed();
    println!("Two: {sum} | Elapsed: {elapsed:?}");
}

fn parse(input: &str) -> Input {
    let (ranges, ids) = input.split_once("\n\n").unwrap();

    let ranges: Vec<_> = ranges
        .lines()
        .map(|range| {
            let (begin, end) = range.split_once("-").unwrap();
            (begin.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect();

    let ids = ids.lines().map(|id| id.parse().unwrap()).collect();
    Input { ranges, ids }
}

fn main() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();
    let input = parse(&input);

    one(input.clone());
    two(input);
}
