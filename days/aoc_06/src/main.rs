use core::num;

#[derive(Debug, Clone, Copy)]
enum Op {
    Num(i64),
    Addition,
    Multiplication,
}

impl Op {
    fn num_or_panic(&self) -> &i64 {
        match self {
            Op::Num(num) => num,
            _ => panic!("'{self}' is not num"),
        }
    }
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Op::Num(num) => num.to_string(),
            Op::Addition => "+".into(),
            Op::Multiplication => "*".into(),
        };
        write!(f, "{output}")
    }
}

type InputOne = Vec<Vec<Op>>;

fn one(input: Vec<String>) {
    let now = std::time::Instant::now();
    let input: Vec<Vec<Op>> = parse_one(&input);
    let input = aoc_lib::transpose2(input);

    let sum: i64 = input
        .into_iter()
        .map(|mut ops| {
            // Rotate CCW makes last element = operand
            match ops.pop().unwrap() {
                Op::Addition => ops.iter().map(|op| op.num_or_panic()).sum::<i64>(),
                Op::Multiplication => ops.iter().map(|op| op.num_or_panic()).product::<i64>(),
                op => panic!("Last element '{op}' is not operand"),
            }
        })
        .sum();

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}");
}

fn parse_one(input: &[String]) -> InputOne {
    input
        .iter()
        .map(|row| row.split_ascii_whitespace())
        .map(|elems| {
            elems
                .into_iter()
                .map(|elem| match elem {
                    "+" => Op::Addition,
                    "*" => Op::Multiplication,
                    num => Op::Num(num.parse().unwrap()),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

type InputTwo = Vec<Vec<String>>;

fn two(input: Vec<String>) {
    let now = std::time::Instant::now();
    let input = parse_two(input);
    for row in &input {
        println!("{row:?}")
    }
    let input = aoc_lib::transpose2(input);
    println!("{input:?}");

    // let sum: i64 = input
    //     .into_iter()
    //     .map(|mut ops| {
    //         // Rotate CCW makes last element = operand
    //         match ops.pop().unwrap() {
    //             Op::Addition => ops.iter().map(|op| op.num_or_panic()).sum::<i64>(),
    //             Op::Multiplication => ops.iter().map(|op| op.num_or_panic()).product::<i64>(),
    //             op => panic!("Last element '{op}' is not operand"),
    //         }
    //     })
    //     .sum();

    let elapsed = now.elapsed();
    // println!("Two: {sum} | Elapsed: {elapsed:?}");
}

// Nasty but works.
fn parse_two(mut input: Vec<String>) -> InputTwo {
    let operands = input.pop().unwrap();
    let mut number_starts: Vec<_> = operands
        .char_indices()
        .filter(|(_, elem)| elem != &' ')
        .collect();
    // Manually add the ending boundary usng the length.
    number_starts.push((operands.len(), '#'));

    let mut nums = Vec::new();

    for window in number_starts[..number_starts.len()].windows(2) {
        let mut col = Vec::new();

        let num = window[0];
        let next = window[1];
        let size = next.0 - num.0;
        for row in input.iter_mut() {
            let rest_of_string = row.split_off(size);

            col.push(row.clone());
            *row = rest_of_string
        }
        nums.push((num.1, col));
    }

    // Remove trailing whitespace on all rows but last.
    let num_rows = nums.len();
    for (_, row) in nums[..num_rows - 1].iter_mut() {
        for num in row {
            num.pop().unwrap();
        }
    }

    for row in nums {
        println!("{row:?}");
    }

    todo!()
}

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();

    one(input.clone());
    two(input);
}
