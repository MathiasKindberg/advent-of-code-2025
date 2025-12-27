type Input = Vec<String>;

fn one(input: Vec<String>) {
    let now = std::time::Instant::now();
    let red_tiles: Vec<(i64, i64)> = input
        .iter()
        .map(|row| row.split_once(',').unwrap())
        .map(|(col, row)| (col.parse().unwrap(), row.parse().unwrap()))
        .collect();

    let mut max_area: i64 = 0;

    for (i, tile_1) in red_tiles.iter().enumerate() {
        for tile_2 in red_tiles[i + 1..].iter() {
            // +1 since we are creating an inclusive area.
            let col_diff = (tile_2.0 - tile_1.0).abs() + 1;
            let row_diff = (tile_2.1 - tile_1.1).abs() + 1;
            let area = col_diff * row_diff;
            if area > max_area {
                max_area = area
            }
        }
    }

    let elapsed = now.elapsed();
    println!("One: {max_area} | Elapsed: {elapsed:?}");
}

fn two(input: Vec<String>) {
    let now = std::time::Instant::now();
    let sum = 0;

    // Add a copy of the first element to the vec to get a continous loop.
    // input.push(input.first().unwrap().clone());

    let red_tiles: Vec<Point> = input
        .iter()
        .map(|row| row.split_once(',').unwrap())
        .map(|(col, row)| Point {
            x: col.parse().unwrap(),
            y: row.parse().unwrap(),
        })
        .collect();

    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: i64,
        y: i64,
    }

    #[derive(Debug)]
    struct BoundedArea {
        tile_1: Point,
        tile_2: Point,
        area: i64,
    }

    let mut bounded_areas: Vec<_> = red_tiles
        .iter()
        .enumerate()
        .flat_map(|(i, tile_1)| {
            red_tiles[i + 1..].iter().map(|tile_2| {
                let x_diff = (tile_2.x - tile_1.x).abs() + 1;
                let y_diff = (tile_2.y - tile_1.y).abs() + 1;
                BoundedArea {
                    tile_1: *tile_1,
                    tile_2: *tile_2,
                    area: x_diff * y_diff,
                }
            })
        })
        .collect();

    bounded_areas.sort_unstable_by(|a, b| b.area.cmp(&a.area));
    println!("{bounded_areas:#?}");

    let min_col = red_tiles.iter().map(|tile| tile.x).min().unwrap();
    let max_col = red_tiles.iter().map(|tile| tile.x).min().unwrap();
    let min_row = red_tiles.iter().map(|tile| tile.y).min().unwrap();
    let max_row = red_tiles.iter().map(|tile| tile.y).min().unwrap();

    println!("{min_col} - {max_col} {min_row} {max_row}");

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

    one(input.clone());
    two(input);
}
