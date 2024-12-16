// cargo run  --bin day16_part1
// cargo test --bin day16_part1

fn main() {
    let input = include_str!("../././input/day16.txt");
    let output = solve(input);
    println!("Day16 part1: {output}");
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
    visited: Vec<Vec<bool>>,
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

        let width = grid.first().unwrap().len();
        let height = grid.len();

        Map {
            grid,
            start: (start_x as u16, start_y as u16),
            visited: vec![vec![false; width]; height],
        }
    }

    fn get_cost(&mut self) -> Cost {
        let mut costs: Vec<Cost> = Vec::new();
        self.find_costs(self.start, Direction::Right, 0, &mut costs);

        costs.sort_unstable();

        *costs.first().unwrap()
    }

    fn find_costs(
        &mut self,
        point: Point,
        direction: Direction,
        curent_cost: Cost,
        costs: &mut Vec<Cost>,
    ) {
        self.set_visited(point, true);

        let forward = direction.move_point(point);
        if !self.get_visited(forward) {
            let cost = curent_cost + 1;

            match self.get(forward) {
                Tile::FLoor => {
                    self.find_costs(forward, direction, cost, costs);
                }
                Tile::End => {
                    costs.push(cost);
                }
                _ => {}
            }
        }

        let clockwise = direction.rotate_clockwise().move_point(point);
        if !self.get_visited(clockwise) {
            let cost = curent_cost + 1 + 1000;

            match self.get(clockwise) {
                Tile::FLoor => {
                    self.find_costs(clockwise, direction.rotate_clockwise(), cost, costs);
                }
                Tile::End => {
                    costs.push(cost);
                }
                _ => {}
            }
        }

        let counterclockwise = direction.rotate_counterclockwise().move_point(point);
        if !self.get_visited(counterclockwise) {
            let cost = curent_cost + 1 + 1000;

            match self.get(counterclockwise) {
                Tile::FLoor => {
                    self.find_costs(
                        counterclockwise,
                        direction.rotate_counterclockwise(),
                        cost,
                        costs,
                    );
                }
                Tile::End => {
                    costs.push(cost);
                }
                _ => {}
            }
        }

        self.set_visited(point, false);
    }

    fn get(&self, point: Point) -> Tile {
        let (x, y) = point;
        self.grid[y as usize][x as usize]
    }

    fn get_visited(&self, point: Point) -> bool {
        let (x, y) = point;
        self.visited[y as usize][x as usize]
    }

    fn set_visited(&mut self, point: Point, bool: bool) {
        let (x, y) = point;
        self.visited[y as usize][x as usize] = bool;
    }
}

fn solve(input: &str) -> Cost {
    let mut map = Map::from_string(input);

    let score: Cost = map.get_cost();

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_part1_main_example() {
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
        assert_eq!(output, 11048)
    }

    #[test]
    fn day16_part1_example2() {
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
        assert_eq!(output, 7036)
    }
}
