use std::iter;

type Input = Vec<String>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn distance_to(&self, dst: &Point) -> f64 {
        (((dst.x - self.x).pow(2) + (dst.y - self.y).pow(2) + (dst.z - self.z).pow(2)) as f64)
            .sqrt()
    }
}

fn one(input: Vec<String>) {
    let now = std::time::Instant::now();

    let points: Vec<_> = input
        .iter()
        .map(|row| {
            let mut iterator = row.split(',');
            Point {
                x: iterator.next().unwrap().parse().unwrap(),
                y: iterator.next().unwrap().parse().unwrap(),
                z: iterator.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let parsing_elapsed = now.elapsed();

    let mut distance_list = Vec::with_capacity(points.len() * points.len());
    let mut visited = std::collections::HashSet::new();
    for point_1 in &points {
        for point_2 in &points {
            if point_1 != point_2 && !visited.contains(&(point_1, point_2)) {
                distance_list.push((point_1, point_2, point_2.distance_to(point_1)));
                visited.insert((point_2, point_1));
            }
        }
    }
    let distances_elapsed = now.elapsed();

    distance_list.sort_unstable_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let sorting_elapsed = now.elapsed();

    // I suppose this could be done with an adjacency list instead and then just step through it. But this is more fun.
    let mut circuits = std::collections::HashMap::new();
    let mut curr_circuit_id = 0;
    for (point_1, point_2, _) in &distance_list[0..1000] {
        let point_1_circuit = circuits.get(point_1).copied();
        let point_2_circuit = circuits.get(point_2).copied();
        match (point_1_circuit, point_2_circuit) {
            (None, None) => {
                circuits.insert(point_1, curr_circuit_id);
                circuits.insert(point_2, curr_circuit_id);
                curr_circuit_id += 1;
            }
            (None, Some(circuit_id)) => _ = circuits.insert(point_1, circuit_id),
            (Some(circuit_id), None) => _ = circuits.insert(point_2, circuit_id),
            // Join the networks.
            (Some(circuit_id_1), Some(circuit_id_2)) => {
                for (_, circuit_id) in circuits.iter_mut() {
                    if *circuit_id == circuit_id_2 {
                        *circuit_id = circuit_id_1
                    }
                }
            }
        }
    }

    let circuit_joining_elapsed = now.elapsed();

    let mut connections = vec![0; circuits.len()];
    for (_, circuit_id) in circuits {
        connections[circuit_id] += 1
    }

    connections.sort_unstable();
    connections.reverse();

    let sum: u64 = connections[0..3].iter().product();

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}");
    println!(
        "parsing_elapsed: {parsing_elapsed:?} distances_elapsed:{distances_elapsed:?} sorting_elapsed: {sorting_elapsed:?} circuit_joining_elapsed {circuit_joining_elapsed:?}"
    )
}

fn two(input: Vec<String>) {
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
