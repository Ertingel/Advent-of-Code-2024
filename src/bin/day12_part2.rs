// cargo run  --bin day12_part2
// cargo test --bin day12_part2

use std::ops;

fn main() {
    let input = include_str!("../././input/day12.txt");
    let output = solve(input);
    println!("Day12 part2: {output}");
}

#[derive(Debug, Clone, Copy)]
struct FieldData {
    char: char,
    area: u16,
    perimeter: u16,
}

impl FieldData {
    fn new(char: char, area: u16, perimeter: u16) -> Self {
        FieldData {
            char,
            area,
            perimeter,
        }
    }

    fn price(&self) -> u32 {
        self.area as u32 * self.perimeter as u32
    }
}

impl ops::Add<FieldData> for FieldData {
    type Output = FieldData;

    fn add(self, _rhs: FieldData) -> FieldData {
        assert_eq!(self.char, _rhs.char);

        FieldData {
            char: self.char,
            area: self.area + _rhs.area,
            perimeter: self.perimeter + _rhs.perimeter,
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    data: Vec<Vec<char>>,
}

impl Map {
    fn from_text(input: &str) -> Self {
        Map {
            data: input.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    fn get_field(&self, checked: &mut [Vec<bool>], x: i16, y: i16, char: char) -> FieldData {
        if !self.matches(x, y, char) {
            return FieldData::new(char, 0, 1);
        }

        if checked[y as usize][x as usize] {
            return FieldData::new(char, 0, 0);
        }

        checked[y as usize][x as usize] = true;

        FieldData::new(char, 1, 0)
            + self.get_field(checked, x + 1, y, char)
            + self.get_field(checked, x - 1, y, char)
            + self.get_field(checked, x, y + 1, char)
            + self.get_field(checked, x, y - 1, char)
    }

    fn get_fields(&self) -> Vec<FieldData> {
        let mut out: Vec<FieldData> = Vec::new();
        let mut checked = vec![vec![false; self.width()]; self.height()];

        for y in 0..self.height() {
            for x in 0..self.width() {
                if checked[y][x] {
                    continue;
                }

                out.push(self.get_field(
                    &mut checked,
                    x as i16,
                    y as i16,
                    self.get(x as i16, y as i16).unwrap(),
                ));
            }
        }

        out
    }

    fn matches(&self, x: i16, y: i16, char: char) -> bool {
        if let Some(c) = self.get(x, y) {
            return char == c;
        }
        false
    }

    fn get(&self, x: i16, y: i16) -> Option<char> {
        if x < 0 || y < 0 {
            return None;
        }

        let out = self.data.get(y as usize)?.get(x as usize)?;

        Some(*out)
    }

    fn width(&self) -> usize {
        self.data.first().unwrap().len()
    }

    fn height(&self) -> usize {
        self.data.len()
    }
}

fn solve(input: &str) -> u32 {
    let map = Map::from_text(input);
    let fields = map.get_fields();

    let total: u32 = fields.iter().map(|field| field.price()).sum();

    //print!("{:?}", fields);

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12_part2_main_example() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let output = solve(input);
        assert_eq!(output, 1206)
    }

    #[test]
    fn day12_part2_example2() {
        let input = "AAAA
BBCD
BBCC
EEEC";
        let output = solve(input);
        assert_eq!(output, 80)
    }

    #[test]
    fn day12_part2_example3() {
        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        let output = solve(input);
        assert_eq!(output, 436)
    }

    #[test]
    fn day12_part2_example4() {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        let output = solve(input);
        assert_eq!(output, 236)
    }

    #[test]
    fn day12_part2_example5() {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        let output = solve(input);
        assert_eq!(output, 368)
    }
}
