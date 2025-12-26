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

    fn distance_squared(&self, dst: &Point) -> i64 {
        (dst.x - self.x).pow(2) + (dst.y - self.y).pow(2) + (dst.z - self.z).pow(2)
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

    let n = points.len();
    let mut heap = std::collections::BinaryHeap::with_capacity(n * (n - 1) / 2);
    for (i, point_1) in points.iter().enumerate() {
        for (j, point_2) in points[i + 1..].iter().enumerate() {
            // BinaryHeap is max-heap, so use Reverse to get min-heap behavior
            heap.push((
                std::cmp::Reverse(point_2.distance_squared(point_1)),
                i,
                i + 1 + j,
            ));
        }
    }

    let parsing_sorting_elapsed = now.elapsed();

    // start with a set of disjoint circuits.
    let mut circuits: Vec<usize> = (0..n).collect();

    let mut circuit_id_cache: std::collections::HashMap<_, _> =
        (0..n).map(|i| (i, vec![i])).collect();

    while let Some((_, idx1, idx2)) = heap.pop() {
        let point_1_circuit_id = circuits[idx1];
        let point_2_circuit_id = circuits[idx2];

        // If disjoint, then join them.
        if point_1_circuit_id != point_2_circuit_id {
            // Perform joining by using the cache and then setting the value.

            // Grab the vector and remove it from the cache.
            let to_copy_in = circuit_id_cache.remove(&point_2_circuit_id).unwrap();

            for point_idx in to_copy_in {
                // Assign network id
                circuits[point_idx] = point_1_circuit_id;

                // Extend cache
                let items = circuit_id_cache.get_mut(&point_1_circuit_id).unwrap();
                items.push(point_idx);
            }

            // If we are down to 1 item in the cache then all ciruits are connected.
            if circuit_id_cache.len() == 1 {
                let sum = points[idx1].x * points[idx2].x;
                let elapsed = now.elapsed();
                println!(
                    "Two: {sum} | Elapsed: {elapsed:?} Sorting and parsing: {parsing_sorting_elapsed:?}"
                );
                break;
            }
        }
    }
}

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let input: Vec<String> = stdin.lock().lines().map_while(Result::ok).collect();

    one(input.clone());
    two(input);
}
