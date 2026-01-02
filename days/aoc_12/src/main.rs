use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Point {
    Occupied,
    Empty,
    OutOfBounds,
}

impl Point {
    fn display_character(&self) -> char {
        match self {
            Self::Occupied => '#',
            Self::Empty => '.',
            Self::OutOfBounds => '@',
        }
    }
}

/// Specialized implementation for 3x3 areas.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Shape {
    shape: [[Point; 3]; 3],
}

impl Shape {
    pub fn new(shape: [[Point; 3]; 3]) -> Self {
        Self { shape }
    }

    pub fn area(&self) -> usize {
        self.shape
            .iter()
            .flat_map(|row| row.iter())
            .filter(|point| matches!(point, Point::Occupied))
            .count()
    }

    ///      ###      .##
    /// ---- ##. ---> ##. ----
    ///      .##      ###
    pub fn flip_horizontal_axis(&mut self) {
        let rows = self.shape.len();
        for i in 0..rows / 2 {
            self.shape.swap(i, rows - 1 - i);
        }
    }

    /// ###   |   ###
    /// #.. ----> ..#
    /// ###   |   ###
    pub fn flip_vertical_axis(&mut self) {
        for row in self.shape.iter_mut() {
            row.reverse();
        }
    }

    /// Attempts to place the shape in the grid as per its current layout.
    /// If it is possible mutates the grid
    pub fn remove_from_grid(&self, mid_x: usize, mid_y: usize, grid: &mut Vec<Vec<Point>>) -> bool {
        // Second pass: place the shape
        for sy in 0..3 {
            for sx in 0..3 {
                if matches!(self.shape[sy][sx], Point::Occupied) {
                    let grid_y = mid_y + sy - 1;
                    let grid_x = mid_x + sx - 1;
                    grid[grid_y][grid_x] = Point::Empty;
                }
            }
        }

        true
    }

    /// Attempts to place the shape in the grid as per its current layout.
    /// If it is possible mutates the grid
    pub fn try_place_in_grid(
        &self,
        mid_x: usize,
        mid_y: usize,
        grid: &mut Vec<Vec<Point>>,
    ) -> bool {
        // First pass: check if placement is possible
        for sy in 0..3 {
            for sx in 0..3 {
                if matches!(self.shape[sy][sx], Point::Occupied) {
                    let grid_y = mid_y + sy - 1;
                    let grid_x = mid_x + sx - 1;
                    if !matches!(grid[grid_y][grid_x], Point::Empty) {
                        return false;
                    }
                }
            }
        }

        // Second pass: place the shape
        for sy in 0..3 {
            for sx in 0..3 {
                if matches!(self.shape[sy][sx], Point::Occupied) {
                    let grid_y = mid_y + sy - 1;
                    let grid_x = mid_x + sx - 1;
                    grid[grid_y][grid_x] = Point::Occupied;
                }
            }
        }

        true
    }
}

impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::with_capacity(self.shape[0].len() * self.shape.len());

        for (idx, row) in self.shape.iter().enumerate() {
            for c in row {
                res.push(c.display_character());
            }

            if idx != self.shape.len() - 1 {
                res.push('\n');
            }
        }
        write!(f, "{res}",)
    }
}
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_character(),)
    }
}

fn parse_one(input: String) -> (Vec<Shape>, Vec<((usize, usize), Vec<usize>)>) {
    let mut input: Vec<_> = input.split("\n\n").collect();
    let regions = input.pop().unwrap();

    (
        input
            .into_iter()
            // Discard the id, it doesn't matter.
            .map(|shape| shape.split_once(":\n").unwrap().1)
            .map(|shape| {
                Shape::new(
                    shape
                        .split("\n")
                        .map(|shape_row| {
                            shape_row
                                .chars()
                                .map(|char| match char {
                                    '#' => Point::Occupied,
                                    '.' => Point::Empty,
                                    unknown => unreachable!("{unknown}"),
                                })
                                .collect::<Vec<_>>()
                                .try_into()
                                .unwrap()
                        })
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                )
            })
            .collect(),
        regions
            .split("\n")
            .map(|region| region.split_once(": ").unwrap())
            .map(|(region, assignment)| {
                (
                    region
                        .split_once("x")
                        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                        .unwrap(),
                    assignment
                        .split_ascii_whitespace()
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect::<Vec<_>>(),
                )
            })
            .collect(),
    )
}

// Return true if all shapes are placed. Enormous search space so does not complete for test input.
pub fn search(
    mut shapes_to_place: Vec<Shape>,
    grid: &mut Vec<Vec<Point>>,
    memoization: &mut std::collections::HashMap<(Vec<Shape>, Vec<Vec<Point>>), bool>,
) -> bool {
    let key = (shapes_to_place.clone(), grid.clone());

    // Check if we've already determined this state is unsolvable
    if let Some(&result) = memoization.get(&key) {
        return result;
    }

    if let Some(mut shape) = shapes_to_place.pop() {
        // Try placing the shape in all locations possibe locations.
        for mid_y in 2..grid.len() - 2 {
            for mid_x in 2..grid[0].len() - 2 {
                // Try all orientations of the 3x3 shape:
                for _ in 0..2 {
                    for _ in 0..4 {
                        if shape.try_place_in_grid(mid_x, mid_y, grid) {
                            if search(shapes_to_place.clone(), grid, memoization) {
                                return true;
                            }
                            shape.remove_from_grid(mid_x, mid_y, grid);
                        }
                        aoc_lib::rotate_90_cw_2d_array_new(&mut shape.shape);
                    }
                    shape.flip_horizontal_axis();
                }
            }
        }
        memoization.insert(key, false);
    } else {
        // Shapes to place = empty, all has been placed correctly.
        return true;
    }
    false
}

fn one(input: String) {
    let now = std::time::Instant::now();
    let mut sum = 0;

    let (shapes, regions) = parse_one(input);

    // We can place the shapes in any order. As long as we try all rotations and flips on all locations for all shapes.
    for ((x, y), shapes_to_place) in regions {
        let shapes_to_place: Vec<_> = shapes_to_place
            .iter()
            .enumerate()
            .flat_map(|(idx, num)| {
                (0..*num)
                    .into_iter()
                    .map(|_| shapes[idx].clone())
                    .collect::<Vec<_>>()
            })
            .collect();

        // Pad input so we don't have to care about indexing out of bounds.
        let mut _grid = aoc_lib::pad_input(vec![vec![Point::Empty; x]; y], Point::OutOfBounds);

        // Check if we can even place all shapes.
        let grid_area = x * y;
        let shape_area: usize = shapes_to_place.iter().map(|shape| shape.area()).sum();

        if shape_area < grid_area {
            sum += 1;
        }

        // DFS searching all possibilites. Too slow to complete.
        // if search(
        //     shapes_to_place,
        //     &mut grid,
        //     &mut std::collections::HashMap::new(),
        // ) {
        //     sum += 1;
        // }
    }

    let elapsed = now.elapsed();
    println!("One: {sum} | Elapsed: {elapsed:?}");
}

fn two(_input: String) {
    let now = std::time::Instant::now();
    let sum = 0;

    let elapsed = now.elapsed();
    println!("Two: {sum} | Elapsed: {elapsed:?}");
}

fn main() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();

    one(input.clone());
    two(input);
}
