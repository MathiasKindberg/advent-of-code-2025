fn one(_input: Vec<String>) {
    let now = std::time::Instant::now();
    let sum = 0;

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}");
}

fn two(_input: Vec<String>) {
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
    two(input);
}
