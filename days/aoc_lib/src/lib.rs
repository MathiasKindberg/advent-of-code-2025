// /// Adds a padding layer of dots around the schematic ensuring
// /// we do not have to deal with the edges.
// pub fn pad_input(mut input: Vec<Vec<char>>, character: char) -> Vec<Vec<char>> {
//     assert!(!input.is_empty(), "Expected input");

//     for row in input.iter_mut() {
//         row.insert(0, character);
//         row.push(character)
//     }

//     let top_bottom_padding: Vec<_> = character
//         .to_string()
//         .repeat(input[0].len())
//         .chars()
//         .collect();
//     input.insert(0, top_bottom_padding.clone());
//     input.push(top_bottom_padding);

//     input
// }

/// Adds a padding layer of dots around the schematic ensuring
/// we do not have to deal with the edges.
pub fn pad_input<T: Clone>(mut input: Vec<Vec<T>>, character: T) -> Vec<Vec<T>> {
    assert!(!input.is_empty(), "Expected input");

    for row in input.iter_mut() {
        row.insert(0, character.clone());
        row.push(character.clone())
    }

    let top_bottom_padding: Vec<_> = vec![character; input[0].len()];

    input.insert(0, top_bottom_padding.clone());
    input.push(top_bottom_padding);

    input
}

// https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
pub fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

/// Reverses all vectors in 2d Vec
pub fn rev2<T>(v: &mut Vec<Vec<T>>) {
    for row in v {
        row.reverse()
    }
}

pub fn rotate_90_ccw_2d<T>(matrix: &mut [Vec<T>])
where
    T: Copy,
{
    let size = matrix.len();

    for x in 0..(size / 2) {
        for y in x..size - x - 1 {
            let temp = matrix[x][y];
            matrix[x][y] = matrix[y][size - 1 - x];
            matrix[y][size - 1 - x] = matrix[size - 1 - x][size - 1 - y];
            matrix[size - 1 - x][size - 1 - y] = matrix[size - 1 - y][x];
            matrix[size - 1 - y][x] = temp;
        }
    }
}

pub fn rotate_90_cw_2d_array<T, const N: usize>(matrix: &mut [&mut [T]; N])
where
    T: Copy,
{
    let size = matrix.len();

    // Traverse each cycle
    for i in 0..(size / 2) {
        for j in i..(size - i - 1) {
            // Swap elements of each cycle
            // in clockwise direction
            let temp = matrix[i][j];
            matrix[i][j] = matrix[size - 1 - j][i];
            matrix[size - 1 - j][i] = matrix[size - 1 - i][size - 1 - j];
            matrix[size - 1 - i][size - 1 - j] = matrix[j][size - 1 - i];
            matrix[j][size - 1 - i] = temp;
        }
    }
}

pub fn rotate_90_cw_2d<T>(matrix: &mut [Vec<T>])
where
    T: Copy,
{
    let size = matrix.len();

    // Traverse each cycle
    for i in 0..(size / 2) {
        for j in i..(size - i - 1) {
            // Swap elements of each cycle
            // in clockwise direction
            let temp = matrix[i][j];
            matrix[i][j] = matrix[size - 1 - j][i];
            matrix[size - 1 - j][i] = matrix[size - 1 - i][size - 1 - j];
            matrix[size - 1 - i][size - 1 - j] = matrix[j][size - 1 - i];
            matrix[j][size - 1 - i] = temp;
        }
    }
}

pub fn print_2d<T>(v: &[Vec<T>])
where
    T: std::fmt::Display,
{
    for row in v {
        for c in row {
            print!("{c}");
        }
        println!()
    }
}

pub fn print_2d_array_mut<T>(v: &[&mut [T]])
where
    T: std::fmt::Display,
{
    for row in v {
        for c in row.iter() {
            print!("{c}");
        }
        println!()
    }
}

pub fn print_2d_spacious<T>(v: &[Vec<T>])
where
    T: std::fmt::Display,
{
    for row in v {
        for c in row {
            print!("{c} ");
        }
        println!()
    }
}

// Could use libraries but this is easy enough.
// https://rosettacode.org/wiki/Least_common_multiple
pub fn gcd(a: usize, b: usize) -> usize {
    use std::cmp::{max, min};

    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

// https://rosettacode.org/wiki/Least_common_multiple
pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
