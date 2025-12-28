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

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

/// Sign of cross product (p2-p1) × (p3-p1)
/// Returns: 1 = CCW, -1 = CW, 0 = collinear
fn orientation(p1: &Point, p2: &Point, p3: &Point) -> i32 {
    let cross = (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x);
    match cross.cmp(&0) {
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
    }
}

/// Check if point P lies on segment AB
fn point_on_segment(p: &Point, a: &Point, b: &Point) -> bool {
    orientation(a, p, b) == 0
        && p.x >= a.x.min(b.x)
        && p.x <= a.x.max(b.x)
        && p.y >= a.y.min(b.y)
        && p.y <= a.y.max(b.y)
}

/// Ray casting. Check if point is inside or on edge of polygon
fn ray_cast_point_in_polygon(point: &Point, polygon: &[Point]) -> bool {
    let mut inside = false;

    for (a, b) in polygon.iter().zip(polygon.iter().cycle().skip(1)) {
        // Point on edge counts as inside
        if point_on_segment(point, a, b) {
            return true;
        }

        // Ray casting: count crossings of ray going right from point
        if (a.y > point.y) != (b.y > point.y) {
            // Edge crosses horizontal line y = point.y
            // Check if crossing is to the right of point using integer math:
            // x_intersect = a.x + (point.y - a.y) * (b.x - a.x) / (b.y - a.y)
            // point.x < x_intersect  ⟺  (point.x - a.x)(b.y - a.y) < (point.y - a.y)(b.x - a.x) when b.y > a.y
            let dy = b.y - a.y;
            let lhs = (point.x - a.x) * dy;
            let rhs = (point.y - a.y) * (b.x - a.x);

            let crossing_right = if dy > 0 { lhs < rhs } else { lhs > rhs };
            if crossing_right {
                inside = !inside;
            }
        }
    }

    inside
}

fn is_rectangle_in_polygon(rectangle: &Rectangle, polygon: &[Point]) -> bool {
    let corners = rectangle.corners();

    for &corner in &corners {
        if !ray_cast_point_in_polygon(&corner, polygon) {
            return false;
        }
    }

    let rect_edges = rectangle.edges();

    // No polygon edge should cross any rectangle edge (but overlapping edges are okay)
    for poly_edge in polygon.iter().zip(polygon.iter().cycle().skip(1)) {
        for &(r1, r2) in &rect_edges {
            // Only check for proper crossings, not collinear overlaps
            let o1 = orientation(poly_edge.0, poly_edge.1, &r1);
            let o2 = orientation(poly_edge.0, poly_edge.1, &r2);
            let o3 = orientation(&r1, &r2, poly_edge.0);
            let o4 = orientation(&r1, &r2, poly_edge.1);

            // Segments properly cross if each straddles the line containing the other
            // Skip any case where segments touch at endpoints or overlap (o == 0)
            if o1 != 0 && o2 != 0 && o3 != 0 && o4 != 0 && o1 != o2 && o3 != o4 {
                return false;
            }
        }
    }

    true
}

#[derive(Debug)]
struct Rectangle {
    min: Point,
    max: Point,
    area: i64,
}

impl Rectangle {
    fn new(p1: Point, p2: Point) -> Self {
        let min = Point {
            x: p1.x.min(p2.x),
            y: p1.y.min(p2.y),
        };
        let max = Point {
            x: p1.x.max(p2.x),
            y: p1.y.max(p2.y),
        };

        // +1 since we are inclusive.
        Self {
            min,
            max,
            area: (max.x - min.x + 1) * (max.y - min.y + 1),
        }
    }

    fn corners(&self) -> [Point; 4] {
        [
            Point {
                x: self.min.x,
                y: self.min.y,
            },
            Point {
                x: self.max.x,
                y: self.min.y,
            },
            Point {
                x: self.max.x,
                y: self.max.y,
            },
            Point {
                x: self.min.x,
                y: self.max.y,
            },
        ]
    }

    fn edges(&self) -> [(Point, Point); 4] {
        let corners = self.corners();
        [
            (corners[0], corners[1]),
            (corners[1], corners[2]),
            (corners[2], corners[3]),
            (corners[3], corners[0]),
        ]
    }
}

fn two(input: Vec<String>) {
    let now = std::time::Instant::now();

    let polygon: Vec<Point> = input
        .iter()
        .map(|row| row.split_once(',').unwrap())
        .map(|(col, row)| Point {
            x: col.parse().unwrap(),
            y: row.parse().unwrap(),
        })
        .collect();

    let mut rectangles: Vec<_> = polygon
        .iter()
        .enumerate()
        .flat_map(|(i, tile_1)| {
            polygon[i + 1..]
                .iter()
                .map(|tile_2| Rectangle::new(*tile_1, *tile_2))
        })
        .collect();

    rectangles.sort_unstable_by(|a, b| b.area.cmp(&a.area));

    for rectangle in rectangles {
        if is_rectangle_in_polygon(&rectangle, &polygon) {
            let elapsed = now.elapsed();
            println!("Two: {} | Elapsed: {elapsed:?}", rectangle.area);
            break;
        }
    }
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
