// cargo run  --bin day04_part1
// cargo test --bin day04_part1

//use advent_of_code::parsing;

fn main() {
    let input = include_str!("../././input/day04.txt");
    let output = solve(input);
    println!("Day04 part1: {output}");
}

fn find(table: &Vec<Vec<char>>, point: &(i32, i32)) -> i32 {
    let directions = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        //(0, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    directions
        .iter()
        .filter(|direction| walk(table, "XMAS", point, direction).is_some())
        .count() as i32
}

fn walk(
    table: &Vec<Vec<char>>,
    word: &str,
    point: &(i32, i32),
    direction: &(i32, i32),
) -> Option<()> {
    let (x, y) = *point;

    let mut itter = word.chars();
    let char = itter.next()?;
    let remaining = &word[char.len_utf8()..];

    let y_usize: usize = y.try_into().ok()?;
    let x_usize: usize = x.try_into().ok()?;
    let char_at: &Vec<char> = table.get(y_usize)?;
    let char_at: char = *char_at.get(x_usize)?;

    if char_at != char {
        return None;
    }

    if remaining.is_empty() {
        return Some(());
    }

    let (dx, dy) = direction;
    walk(table, remaining, &(x + dx, y + dy), direction)
}

fn solve(input: &str) -> i32 {
    let table: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
    let width = table[0].len() as i32;
    let height = table.len() as i32;

    let matches: Vec<i32> = (0..width)
        .flat_map(|y2| (0..height).map(|x2| (x2, y2)).collect::<Vec<(i32, i32)>>())
        .map(|point| find(&table, &point))
        .collect();

    let total: i32 = matches.iter().sum();

    //println!("{:?}", matches);
    //println!("{total}");
    /*
    ....XXMAS.
    .SAMXMS...
    ...S..A...
    ..A.A.MS.X
    XMASAMX.MM
    X.....XA.A
    S.S.S.S.SS
    .A.A.A.A.A
    ..M.M.M.MM
    .X.X.XMASX
    */

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day04_part1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let output = solve(input);
        assert_eq!(output, 18)
    }

    #[test]
    fn day04_part1_extra() {
        let input = "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX";
        let output = solve(input);
        assert_eq!(output, 18)
    }
}
