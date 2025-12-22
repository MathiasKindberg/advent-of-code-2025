type Input = Vec<i64>;

fn one(input: &Input) {
    let now = std::time::Instant::now();
    let mut sum = 0;

    let mut direction = 50;
    for line in input {
        direction += line;
        if direction % 100 == 0 {
            sum += 1;
        }
    }

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}");
}
fn two(input: &Input) {
    let now = std::time::Instant::now();
    let mut sum = 0;

    let mut direction = 50;
    for line in input {
        // Capture turning through zero.
        if direction != 0 {
            let mod_direction = direction + (line % 100);
            if !(0..=100).contains(&mod_direction) {
                sum += 1;
            }
        }

        direction += line;

        // Capture turning multiple times
        let div = (line / 100).abs();
        sum += div;

        // Capture landing on zero
        if direction % 100 == 0 {
            sum += 1;
        }

        // Reset dial to within [0, 99] bound
        direction %= 100;
        if direction < 0 {
            direction += 100
        }
    }

    let elapsed = now.elapsed();
    println!("Two: {sum} | Elapsed: {elapsed:?}");
}

fn parse(input: &[String]) -> Input {
    input
        .iter()
        .map(|line| line.split_at(1))
        .map(|(dir, degrees)| {
            let degrees: i64 = degrees.parse().unwrap();
            match dir {
                "R" => degrees,
                "L" => -degrees,
                _ => unreachable!(),
            }
        })
        .collect()
}

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();
    let input = parse(&input);

    one(&input);
    two(&input);
}
