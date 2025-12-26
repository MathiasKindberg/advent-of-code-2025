use itertools::Itertools;

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
    let connections = if input.len() > 100 { 1000 } else { 10 };

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

    let n = points.len();
    let mut distance_list = Vec::with_capacity(n * (n - 1) / 2);
    for (i, point_1) in points.iter().enumerate() {
        for point_2 in &points[i + 1..] {
            distance_list.push((point_1, point_2, point_2.distance_to(point_1)));
        }
    }
    distance_list.sort_unstable_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    // I suppose this could be done with an adjacency list instead and then just step through it. But this is more fun.
    let mut circuits = std::collections::HashMap::new();
    let mut curr_circuit_id = 0;
    for (point_1, point_2, _) in &distance_list[0..connections] {
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

    let mut connections = vec![0; circuits.len()];
    for (_, circuit_id) in circuits {
        connections[circuit_id] += 1
    }

    // a, b reversed to have largest value first.
    connections.sort_unstable_by(|a, b| b.cmp(a));

    let sum: u64 = connections[0..3].iter().product();

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}");
}

fn two(input: Vec<String>) {
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

    let n = points.len();
    let mut distance_list = Vec::with_capacity(n * (n - 1) / 2);
    for (i, point_1) in points.iter().enumerate() {
        for point_2 in &points[i + 1..] {
            distance_list.push((point_1, point_2, point_2.distance_to(point_1)));
        }
    }
    let distances_elapsed = now.elapsed();

    distance_list.sort_unstable_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    println!("last: {:?}", distance_list.last().unwrap());

    let sorting_elapsed = now.elapsed();

    // Now we need to keep track of the number of independent networks, when we are back to zero we are done.
    // let mut circuits = std::collections::HashMap::new();
    // let mut circuits: std::collections::HashMap<_, _> = points
    //     .iter()
    //     .enumerate()
    //     .map(|(circuit, point)| (point, circuit))
    //     .collect();

    let mut circuits: Vec<std::collections::HashSet<_>> = points
        .iter()
        .map(|point| vec![point].into_iter().collect())
        .collect();

    // A location cache to know which circuit to use.
    let circuit_loc_cache: std::collections::HashMap<_, _> = circuits
        .iter()
        .enumerate()
        .map(|(idx, circuit)| (circuit.iter().next().unwrap(), idx))
        .collect();

    for (point_1, point_2, _) in &distance_list {
        let point_1_circuit_id = circuit_loc_cache.get(point_1).unwrap();
        let point_2_circuit_id = circuit_loc_cache.get(point_2).unwrap();
    }
}

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();

    one(input.clone());
    two(input);
}
