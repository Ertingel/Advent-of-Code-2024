// cargo run  --bin day12_part2
// cargo test --bin day12_part2

use std::{collections::HashSet, ops};

fn main() {
    let input = include_str!("../././input/day12.txt");
    let output = solve(input);
    println!("Day12 part2: {output}");
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct FieldData {
    char: char,
    area: u16,
    perimeter: u16,
    sides: u16,
}

impl FieldData {
    fn new(char: char, area: u16, perimeter: u16, sides: u16) -> Self {
        FieldData {
            char,
            area,
            perimeter,
            sides,
        }
    }
    /*
       fn price(&self) -> u32 {
           self.area as u32 * self.perimeter as u32
       }
    */
    fn bulk_price(&self) -> u32 {
        self.area as u32 * self.sides as u32
    }

    fn merge_sides(&mut self, data: HashSet<(i16, i16)>) {
        let data = data.clone();

        let data: HashSet<&(i16, i16)> = data
            .iter()
            .filter(|(x, y)| !data.contains(&(*x + 3, *y)))
            .filter(|(x, y)| !data.contains(&(*x, *y + 3)))
            .collect();

        //println!("{:?}", data);
        /*
        let min_x = *data.iter().map(|(x, _)| x).min().unwrap();
        let max_x = *data.iter().map(|(x, _)| x).max().unwrap();
        let min_y = *data.iter().map(|(_, y)| y).min().unwrap();
        let max_y = *data.iter().map(|(_, y)| y).max().unwrap();

        println!("Fences:");
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if data.contains(&(x, y)) {
                    print!("X");
                } else if x % 3 == 0 && y % 3 == 0 {
                    print!("+");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
        println!();
         */
        self.sides = data.len() as u16;
    }
}

impl ops::Add<FieldData> for FieldData {
    type Output = FieldData;

    fn add(self, rhs: FieldData) -> FieldData {
        assert_eq!(self.char, rhs.char);

        FieldData {
            char: self.char,
            area: self.area + rhs.area,
            perimeter: self.perimeter + rhs.perimeter,
            sides: self.sides + rhs.sides,
        }
    }
}

impl ops::AddAssign<FieldData> for FieldData {
    fn add_assign(&mut self, rhs: FieldData) {
        assert_eq!(self.char, rhs.char);

        self.area += rhs.area;
        self.perimeter += rhs.perimeter;
        self.sides += rhs.sides;
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

    fn get_field(
        &self,
        checked: &mut [Vec<bool>],
        x: i16,
        y: i16,
        char: char,
    ) -> (FieldData, HashSet<(i16, i16)>) {
        let mut total = FieldData::new(char, 1, 0, 0);
        let mut fences: HashSet<(i16, i16)> = HashSet::new();

        checked[y as usize][x as usize] = true;

        if self.matches(x + 1, y, char) {
            if !self.bool_check(checked, x + 1, y) {
                let (field_data, fence_data) = self.get_field(checked, x + 1, y, char);
                total += field_data;
                fences = fences.union(&fence_data).cloned().collect();
            }
        } else {
            fences.insert((x * 3 + 1, y * 3));
            total.perimeter += 1;
        }

        if self.matches(x - 1, y, char) {
            if !self.bool_check(checked, x - 1, y) {
                let (field_data, fence_data) = self.get_field(checked, x - 1, y, char);
                total += field_data;
                fences = fences.union(&fence_data).cloned().collect();
            }
        } else {
            fences.insert((x * 3 - 1, y * 3));
            total.perimeter += 1;
        }

        if self.matches(x, y + 1, char) {
            if !self.bool_check(checked, x, y + 1) {
                let (field_data, fence_data) = self.get_field(checked, x, y + 1, char);
                total += field_data;
                fences = fences.union(&fence_data).cloned().collect();
            }
        } else {
            fences.insert((x * 3, y * 3 + 1));
            total.perimeter += 1;
        }

        if self.matches(x, y - 1, char) {
            if !self.bool_check(checked, x, y - 1) {
                let (field_data, fence_data) = self.get_field(checked, x, y - 1, char);
                total += field_data;
                fences = fences.union(&fence_data).cloned().collect();
            }
        } else {
            fences.insert((x * 3, y * 3 - 1));
            total.perimeter += 1;
        }

        (total, fences)
    }

    fn get_fields(&self) -> Vec<FieldData> {
        let mut out: Vec<FieldData> = Vec::new();
        let mut checked = vec![vec![false; self.width()]; self.height()];

        for y in 0..self.height() {
            for x in 0..self.width() {
                if checked[y][x] {
                    continue;
                }

                let (mut fence_data, sides) = self.get_field(
                    &mut checked,
                    x as i16,
                    y as i16,
                    self.get(x as i16, y as i16).unwrap(),
                );

                assert_eq!(
                    fence_data.perimeter,
                    sides.len() as u16,
                    "Perimeter not matching! "
                );

                fence_data.merge_sides(sides);

                out.push(fence_data);
            }
        }

        out
    }

    fn bool_check(&self, checked: &[Vec<bool>], x: i16, y: i16) -> bool {
        if x < 0 || y < 0 || x >= self.width() as i16 || y >= self.height() as i16 {
            return false;
        }
        checked[y as usize][x as usize]
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

    let total: u32 = fields.iter().map(|field| field.bulk_price()).sum();

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

    #[test]
    fn day12_part2_example6() {
        let input = "XX";
        let output = solve(input);
        assert_eq!(output, 8)
    }

    #[test]
    fn day12_part2_example7() {
        let input = "XX
XO";
        let output = solve(input);
        assert_eq!(output, 22)
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12_part2_example6() {
        let input = "XX
XO";
        let output = solve(input);
        assert_eq!(output, 8)
    }
}
 */
