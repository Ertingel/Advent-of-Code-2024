// cargo run  --bin day04_part2
// cargo test --bin day04_part2

fn main() {
    let input = include_str!("../././input/day04.txt");
    let output = solve(input);
    println!("Day04 part2: {output}");
}

fn find(table: &Vec<Vec<char>>, point: &(i32, i32)) -> bool {
    let directions = [
        ((-1, -1), (-1, 1)),
        ((-1, 1), (1, 1)),
        ((1, 1), (1, -1)),
        ((1, -1), (-1, -1)),
    ];

    directions.iter().any(|(direction1, direction2)| {
        let point1 = &(point.0 - direction1.0, point.1 - direction1.1);
        let point2 = &(point.0 - direction2.0, point.1 - direction2.1);

        walk(table, "MAS", point1, direction1).is_some()
            && walk(table, "MAS", point2, direction2).is_some()
    })
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

    let matches: Vec<(i32, i32)> = (0..width)
        .flat_map(|y2| (0..height).map(|x2| (x2, y2)).collect::<Vec<(i32, i32)>>())
        .filter(|point| find(&table, point))
        .collect();

    let total: i32 = matches.len() as i32;

    //println!("{:?}", matches);
    //println!("{total}");
    /*
    .M.S......
    ..A..MSMS.
    .M.S.MAA..
    ..A.ASMSM.
    .M.S.M....
    ..........
    S.S.S.S.S.
    .A.A.A.A..
    M.M.M.M.M.
    ..........
    */

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day04_part2() {
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
        assert_eq!(output, 9)
    }

    #[test]
    fn day04_part2_extra() {
        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        let output = solve(input);
        assert_eq!(output, 9)
    }
}
