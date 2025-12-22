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

type Input = Vec<Vec<Op>>;

fn one(input: Input) {
    let now = std::time::Instant::now();
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

fn two(_input: &Input) {
    let now = std::time::Instant::now();
    let sum = 0;

    let elapsed = now.elapsed();
    println!("Two: {sum} | Elapsed: {elapsed:?}");
}

fn parse(input: &[String]) -> Input {
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

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();
    let input = parse(&input);
    // println!("{input:?}");

    one(input.clone());
    two(&input);
}
