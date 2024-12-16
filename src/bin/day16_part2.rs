// cargo run  --bin day16_part2
// cargo test --bin day16_part2

fn main() {
    let input = include_str!("../././input/day16.txt");
    let output = solve(input);
    println!("Day16 part2: {output}");
}

type Cost = u32;
type Point = (u16, u16);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Upp,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_clockwise(&self) -> Self {
        match self {
            Direction::Upp => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Upp,
            Direction::Right => Direction::Down,
        }
    }

    fn rotate_counterclockwise(&self) -> Self {
        match self {
            Direction::Upp => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Upp,
        }
    }

    fn move_point(&self, point: Point) -> Point {
        let (x, y) = point;
        match self {
            Direction::Upp => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Tile {
    Start,
    End,
    FLoor,
    Wall,
}

impl Tile {
    fn from_char(char: char) -> Self {
        match char {
            'S' => Tile::Start,
            'E' => Tile::End,
            '.' => Tile::FLoor,
            '#' => Tile::Wall,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Map {
    grid: Vec<Vec<Tile>>,
    start: Point,
    end: Point,
    costs: Vec<Vec<Cost>>,
}

impl Map {
    fn from_string(data: &str) -> Map {
        let grid: Vec<Vec<Tile>> = data
            .lines()
            .map(|line| line.chars().map(Tile::from_char).collect())
            .collect();

        let (start_x, start_y, _) = grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, tile)| (x, y, tile))
                    .collect::<Vec<(usize, usize, &Tile)>>()
            })
            .find(|(_, _, tile)| **tile == Tile::Start)
            .unwrap();

        let (end_x, end_y, _) = grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, tile)| (x, y, tile))
                    .collect::<Vec<(usize, usize, &Tile)>>()
            })
            .find(|(_, _, tile)| **tile == Tile::End)
            .unwrap();

        let start = (start_x as u16, start_y as u16);
        let end = (end_x as u16, end_y as u16);

        let width = grid.first().unwrap().len();
        let height = grid.len();

        let mut map = Map {
            grid,
            start,
            end,
            costs: vec![vec![u32::MAX; width]; height],
        };

        map.set_tile(start, Tile::FLoor);
        map.set_tile(end, Tile::FLoor);

        map.fill_costs(start, Direction::Right, 0);

        map
    }

    fn get_end_cost(&self) -> Cost {
        self.get_cost(self.end)
    }

    fn fill_costs(&mut self, point: Point, direction: Direction, curent_cost: Cost) {
        if curent_cost >= self.get_cost(point) {
            return;
        }

        self.set_cost(point, curent_cost);

        let next_direction = direction;
        let next_point = next_direction.move_point(point);
        let next_cost = curent_cost + 1;

        if self.get_tile(next_point) == Tile::FLoor {
            self.fill_costs(next_point, next_direction, next_cost);
        }

        let next_direction = direction.rotate_clockwise();
        let next_point = next_direction.move_point(point);
        let next_cost = curent_cost + 1 + 1000;

        if self.get_tile(next_point) == Tile::FLoor {
            self.fill_costs(next_point, next_direction, next_cost);
        }

        let next_direction = direction.rotate_counterclockwise();
        let next_point = next_direction.move_point(point);
        let next_cost = curent_cost + 1 + 1000;

        if self.get_tile(next_point) == Tile::FLoor {
            self.fill_costs(next_point, next_direction, next_cost);
        }
    }

    fn get_tile(&self, point: Point) -> Tile {
        let (x, y) = point;
        self.grid[y as usize][x as usize]
    }

    fn set_tile(&mut self, point: Point, tile: Tile) {
        let (x, y) = point;
        self.grid[y as usize][x as usize] = tile
    }

    fn get_cost(&self, point: Point) -> Cost {
        let (x, y) = point;
        self.costs[y as usize][x as usize]
    }

    fn set_cost(&mut self, point: Point, cost: Cost) {
        let (x, y) = point;
        self.costs[y as usize][x as usize] = cost;
    }

    /* fn print(&self) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                match tile {
                    Tile::Start => print!("S"),
                    Tile::End => print!("E"),
                    Tile::FLoor => print!(
                        "{}",
                        self.get_cost((x as u16, y as u16))
                            .to_string()
                            .chars()
                            .last()
                            .unwrap_or('.')
                    ),
                    Tile::Wall => print!("#"),
                }
            }
            println!();
        }
        println!()
    } */
}

fn solve(input: &str) -> Cost {
    let map = Map::from_string(input);
    //map.print();

    let score: Cost = map.get_end_cost();

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_part2_main_example() {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        let output = solve(input);
        assert_eq!(output, 64)
    }

    #[test]
    fn day16_part2_example2() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        let output = solve(input);
        assert_eq!(output, 45)
    }
}
