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

// Nasty but works. With this method I align each number with the operand. Leaidng to really
// nasty substring parsing and cloning of data.
//
// A much smarter method would be to pop off numbers by the end of the input and then whenver encountering
// an operand do the summing.
fn two(input: Vec<String>) {
    let now = std::time::Instant::now();
    let mut input = parse_two(input);

    let mut sum = 0;
    for (op, row) in input.iter_mut() {
        let mut numbers = Vec::new();
        while let Some(num) = row
            .iter_mut()
            .map(|elem| elem.pop())
            .collect::<Option<String>>()
        {
            if let Ok(number) = num.trim().parse::<u64>() {
                numbers.push(number)
            }
        }
        sum += match op {
            Op::Addition => numbers.into_iter().sum::<u64>(),
            Op::Multiplication => numbers.into_iter().product::<u64>(),
            Op::Num(_) => unreachable!(),
        };
    }

    let elapsed = now.elapsed();
    println!("Two: {sum} | Elapsed: {elapsed:?}");
}

// Nasty but works. Iterate over windows and align it using the operands while
// using the next number to know how long they are, and then include "#" as just
// junk data to signify the last window with the length to know how long the last
// chunk of data is.
//
// Yes. Awful.
fn parse_two(mut input: Vec<String>) -> Vec<(Op, Vec<String>)> {
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

        nums.push((
            match num.1 {
                '*' => Op::Multiplication,
                '+' => Op::Addition,
                _ => unreachable!(),
            },
            col,
        ));
    }

    nums
}

fn two_clean(mut input: Vec<String>) {
    let now = std::time::Instant::now();

    let mut sum: u64 = 0;
    let mut buffer: Vec<u64> = Vec::with_capacity(5);

    while let Some(column) = input
        .iter_mut()
        .map(|row| row.pop())
        .collect::<Option<Vec<_>>>()
    {
        // Grab last row for operand
        let operand = &column.last().unwrap();

        // Grab the rest as the nuber
        let num = &column[0..column.len() - 1];

        // To make this more efficient we could build the number ourself directly
        // from the char parts and its location instead of collecting a string.
        // But this is much cleaner code.
        if let Ok(num) = num.iter().collect::<String>().trim().parse::<u64>() {
            buffer.push(num);
        }
        match operand {
            '+' => {
                sum += buffer.iter().sum::<u64>();
                buffer.clear();
            }
            '*' => {
                sum += buffer.iter().product::<u64>();
                buffer.clear();
            }
            ' ' => (),
            op => unreachable!("unknown operand {op}"),
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
    two(input.clone());
    two_clean(input);
}
