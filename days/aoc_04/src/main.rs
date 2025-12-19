type Input = Vec<String>;

fn one(input: &Input) {
    let now = std::time::Instant::now();
    let sum = 0;

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}");
}

fn two(input: &Input) {
    let now = std::time::Instant::now();
    let sum = 0;

    let elapsed = now.elapsed();
    println!("Two: {sum} | Elapsed: {elapsed:?}");
}

fn parse(input: &[String]) -> Input {
    input.iter().map(|row| row.to_owned()).collect()
}

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();
    let input = parse(&input);

    one(&input);
    two(&input);
}
