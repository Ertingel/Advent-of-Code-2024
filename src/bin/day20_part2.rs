// cargo run  --bin day20_part2
// cargo test --bin day20_part2

use std::fmt;

use advent_of_code::grid::Grid;

fn main() {
    let input = include_str!("../././input/day20.txt");
    let output = solve(input);
    println!("Day20 part2: {output}");
}

type Distance = i32;
type Point = (u16, u16);

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

impl fmt::Display for Tile {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::Start => fmt.write_str("S"),
            Tile::End => fmt.write_str("E"),
            Tile::FLoor => fmt.write_str("."),
            Tile::Wall => fmt.write_str("#"),
        }
    }
}

fn flod_fill(tiles: &Grid<Tile>, grid: &mut Grid<Distance>, point: Point, distance: Distance) {
    let (x, y) = point;

    if tiles.get(x, y).unwrap_or(&Tile::Wall) == &Tile::Wall {
        return;
    }

    if grid.get(x, y).unwrap_or(&Distance::MIN) <= &distance {
        return;
    }

    grid.set(x, y, distance);
    let distance = distance + 1;

    flod_fill(tiles, grid, (x - 1, y), distance);
    flod_fill(tiles, grid, (x + 1, y), distance);
    flod_fill(tiles, grid, (x, y - 1), distance);
    flod_fill(tiles, grid, (x, y + 1), distance);
}

fn solve(input: &str) -> usize {
    let map = Grid::map_str(input, |_, _, char| Tile::from_char(char));

    let mut start_distance = Grid::new(map.width(), map.height(), Distance::MAX);
    let mut end_distance = Grid::new(map.width(), map.height(), Distance::MAX);

    let start = map
        .iter_xy()
        .find_map(|(x, y, tile)| {
            if *tile == Tile::Start {
                Some((x as u16, y as u16))
            } else {
                None
            }
        })
        .unwrap();

    let end = map
        .iter_xy()
        .find_map(|(x, y, tile)| {
            if *tile == Tile::End {
                Some((x as u16, y as u16))
            } else {
                None
            }
        })
        .unwrap();

    flod_fill(&map, &mut start_distance, start, 0);
    flod_fill(&map, &mut end_distance, end, 0);

    assert_eq!(
        start_distance.get(end.0, end.1).unwrap(),
        end_distance.get(start.0, start.1).unwrap()
    );
    assert_ne!(start_distance.get(end.0, end.1).unwrap(), &Distance::MAX);
    assert_ne!(end_distance.get(start.0, start.1).unwrap(), &Distance::MAX);

    let normal_distance = start_distance.get(end.0, end.1).unwrap();

    let gained: Grid<Option<Distance>> = map.map(|x, y, tile| {
        if tile != &Tile::Wall {
            return None;
        }
        let seach = [
            (x as i16 - 1, y as i16),
            (x as i16 + 1, y as i16),
            (x as i16, y as i16 - 1),
            (x as i16, y as i16 + 1),
        ];

        let lowest_start = *seach
            .iter()
            .map(|(x, y)| start_distance.get(*x, *y).unwrap_or(&Distance::MAX))
            .min()?;

        let lowest_end = *seach
            .iter()
            .map(|(x, y)| end_distance.get(*x, *y).unwrap_or(&Distance::MAX))
            .min()?;

        if lowest_start == Distance::MAX || lowest_end == Distance::MAX {
            return None;
        }

        Some(normal_distance - (lowest_start + lowest_end) - 2)
    });

    let gained: Vec<Distance> = gained.into_iter().flatten().filter(|x| *x >= 100).collect();
    //let mut gained: Vec<Distance> = gained.into_iter().flatten().filter(|x| *x > 0).collect();
    //gained.sort_unstable();

    //println!("{:?}", gained);

    let total: usize = gained.len();

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day20_part2() {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        let _output = solve(input);
        //assert_eq!(output, 40)
    }
}
