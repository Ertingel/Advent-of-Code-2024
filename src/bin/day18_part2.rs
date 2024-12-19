// cargo run  --bin day18_part2
// cargo test --bin day18_part2

use std::collections::HashMap;

fn main() {
    let input = include_str!("../././input/day18.txt");
    let output = solve(input, 70);
    println!("Day18 part2: {output}");
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T> Grid<T>
where
    T: Clone,
{
    fn new(width: usize, height: usize, value: T) -> Self {
        Grid {
            data: vec![vec![value; width]; height],
        }
    }
}

impl<T> Grid<T> {
    fn get<X, Y>(&self, x: X, y: Y) -> Option<&T>
    where
        X: TryInto<usize>,
        Y: TryInto<usize>,
    {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;

        self.data.get(y)?.get(x)
    }

    fn set<X, Y>(&mut self, x: X, y: Y, value: T) -> Option<()>
    where
        X: TryInto<usize>,
        Y: TryInto<usize>,
    {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;

        *self.data.get_mut(y)?.get_mut(x)? = value;

        Some(())
    }

    fn width(&self) -> usize {
        self.data.first().unwrap().len()
    }

    fn height(&self) -> usize {
        self.data.len()
    }
}

fn fill_distance(
    distance_field: &mut Grid<u32>,
    falling: &HashMap<(i8, i8), u32>,
    fallen: u32,
    x: i8,
    y: i8,
    distance: u32,
) {
    if let Some(dis) = distance_field.get(x, y) {
        if distance >= *dis {
            return;
        }
    } else {
        return;
    }

    if let Some(time) = falling.get(&(x, y)) {
        if *time < fallen {
            return;
        }
    }

    distance_field.set(x, y, distance);

    fill_distance(distance_field, falling, fallen, x - 1, y, distance + 1);
    fill_distance(distance_field, falling, fallen, x + 1, y, distance + 1);
    fill_distance(distance_field, falling, fallen, x, y - 1, distance + 1);
    fill_distance(distance_field, falling, fallen, x, y + 1, distance + 1);
}
/*
fn print_grid(
    distance_field: &Grid<u32>,
    falling: &HashMap<(i8, i8), u32>,
    fallen: u32,
    space: usize,
) {
    for y in 0..=space {
        for x in 0..=space {
            if let Some(time) = falling.get(&(x as i8, y as i8)) {
                if *time < fallen {
                    print!("#");
                } else {
                    print!("{}", distance_field.get(x, y).unwrap() % 10);
                }
            } else {
                print!("{}", distance_field.get(x, y).unwrap() % 10);
            }
        }
        println!()
    }
}
 */

fn test(falling: &HashMap<(i8, i8), u32>, space: usize, fallen: u32) -> bool {
    let mut distance_field = Grid::new(space + 1, space + 1, u32::MAX);

    fill_distance(&mut distance_field, falling, fallen, 0, 0, 0);

    let steps: u32 = *distance_field
        .get(distance_field.width() - 1, distance_field.height() - 1)
        .unwrap();

    steps != u32::MAX
}

fn solve(input: &str, space: usize) -> String {
    let falling: HashMap<(i8, i8), u32> = input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse::<i8>().unwrap();
            let y = y.parse::<i8>().unwrap();

            ((x, y), index as u32)
        })
        .collect();

    let mut tested = falling.len() as u32 / 2;
    let mut step_size = tested / 2;

    while step_size > 1 {
        if test(&falling, space, tested) {
            tested += step_size;
        } else {
            tested -= step_size;
        }

        step_size /= 2;
    }

    //print_grid(&distance_field, &falling, fallen, space);

    let (x, y) = falling
        .iter()
        .find_map(|(key, &val)| if val == tested - 1 { Some(key) } else { None })
        .unwrap();

    x.to_string() + "," + &y.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day18_part2() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        let output = solve(input, 6);
        assert_eq!(output, "6,1")
    }
}
