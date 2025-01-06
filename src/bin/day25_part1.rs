// cargo run  --bin day25_part1
// cargo test --bin day25_part1

use advent_of_code::grid::Grid;

fn main() {
    let input = include_str!("../././input/day25.txt");
    let output = solve(input);
    println!("Day25 part1: {output}");
}

type Key = [u8; 5];

fn parse_input(input: &str) -> (Vec<Key>, Vec<Key>) {
    let mut keys: Vec<Key> = Vec::new();
    let mut locks: Vec<Key> = Vec::new();

    for schematic in input.split("\n\n") {
        let is_lock = schematic.starts_with('#');

        let schematic: Grid<bool> = Grid::map_str(schematic, |_, _, char| char == '#');

        let key: Key = (0..schematic.width())
            .map(|pin| {
                for height in 1..schematic.height() {
                    if schematic.get(pin, height - 1) != schematic.get(pin, height) {
                        return height as u8;
                    }
                }
                panic!()
            })
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        if is_lock {
            locks.push(key);
        } else {
            keys.push(key);
        }
    }

    (keys, locks)
}

fn check_key_lock(key: &Key, lock: &Key) -> bool {
    for (k, l) in key.iter().zip(lock) {
        if k < l {
            return false;
        }
    }
    true
}

fn solve(input: &str) -> u32 {
    let (keys, locks) = parse_input(input);

    let mut total: u32 = 0;

    for key in keys.iter() {
        for lock in locks.iter() {
            if check_key_lock(key, lock) {
                total += 1;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day25_part1() {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        let output = solve(input);
        assert_eq!(output, 3)
    }
}
