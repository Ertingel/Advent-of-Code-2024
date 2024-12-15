// cargo run  --bin day15_part2
// cargo test --bin day15_part2

fn main() {
    let input = include_str!("../././input/day15.txt");
    let output = solve(input);
    println!("Day15 part2: {output}");
}

type Point = (i16, i16);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Instruction {
    Upp,
    Down,
    Left,
    Right,
}

impl Instruction {
    fn from_char(char: char) -> Option<Self> {
        match char {
            '^' => Some(Instruction::Upp),
            'v' => Some(Instruction::Down),
            '<' => Some(Instruction::Left),
            '>' => Some(Instruction::Right),
            _ => None,
        }
    }

    fn move_point(&self, (x, y): Point, amount: i16) -> Point {
        match self {
            Instruction::Upp => (x, y - amount),
            Instruction::Down => (x, y + amount),
            Instruction::Left => (x - amount, y),
            Instruction::Right => (x + amount, y),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum FloorTile {
    BoxLeft,
    BoxRight,
    Nothing,
    Wall,
}

impl FloorTile {
    fn from_char(char: char) -> Option<[Self; 2]> {
        match char {
            'O' => Some([FloorTile::BoxLeft, FloorTile::BoxRight]),
            '.' => Some([FloorTile::Nothing, FloorTile::Nothing]),
            '#' => Some([FloorTile::Wall, FloorTile::Wall]),
            _ => None,
        }
    }

    fn to_char(self) -> char {
        match self {
            FloorTile::BoxLeft => '[',
            FloorTile::BoxRight => ']',
            FloorTile::Nothing => '.',
            FloorTile::Wall => '#',
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Warehouse {
    floor: Vec<Vec<FloorTile>>,
    robot: Point,
    width: i16,
    height: i16,
}

impl Warehouse {
    fn from_string(data: &str) -> (Self, Vec<Instruction>) {
        let (floor, instructions) = data.split_once("\n\n").unwrap();

        let mut robot: Point = (-1, -1);
        let floor: Vec<Vec<FloorTile>> = floor
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .flat_map(|(x, char)| {
                        if char == '@' {
                            robot = (2 * x as i16, y as i16);
                            return [FloorTile::Nothing, FloorTile::Nothing];
                        }
                        FloorTile::from_char(char).unwrap()
                    })
                    .collect()
            })
            .collect();

        let width = floor.first().unwrap().len() as i16;
        let height = floor.len() as i16;

        let instructions: Vec<Instruction> = instructions
            .chars()
            .filter(|char| *char != '\n')
            .map(|char| Instruction::from_char(char).unwrap())
            .collect();

        (
            Warehouse {
                floor,
                robot,
                width,
                height,
            },
            instructions,
        )
    }

    fn execute_instructions(&mut self, instructions: Vec<Instruction>) {
        self.print();

        for instruction in instructions {
            self.move_robot(instruction);
            self.print();
        }
    }

    fn move_robot(&mut self, instruction: Instruction) {
        let next_point = instruction.move_point(self.robot, 1);

        if self.move_check(next_point, instruction) {
            self.push(next_point, instruction);
            self.robot = next_point;
        }
    }

    fn push(&mut self, (x, y): Point, direction: Instruction) {
        let tile = self.get((x, y)).unwrap();
        match (tile, direction) {
            (FloorTile::BoxLeft, Instruction::Right) => {
                self.push((x + 2, y), direction);

                self.set((x, y), FloorTile::Nothing);
                self.set((x + 1, y), FloorTile::BoxLeft);
                self.set((x + 2, y), FloorTile::BoxRight);
            }
            (FloorTile::BoxRight, Instruction::Left) => {
                self.push((x - 2, y), direction);

                self.set((x, y), FloorTile::Nothing);
                self.set((x - 1, y), FloorTile::BoxRight);
                self.set((x - 2, y), FloorTile::BoxLeft);
            }

            (FloorTile::BoxLeft, Instruction::Upp) => {
                self.push((x, y - 1), direction);
                self.push((x + 1, y - 1), direction);

                self.set((x, y), FloorTile::Nothing);
                self.set((x, y - 1), FloorTile::BoxLeft);
                self.set((x + 1, y), FloorTile::Nothing);
                self.set((x + 1, y - 1), FloorTile::BoxRight);
            }
            (FloorTile::BoxRight, Instruction::Upp) => {
                self.push((x, y - 1), direction);
                self.push((x - 1, y - 1), direction);

                self.set((x - 1, y), FloorTile::Nothing);
                self.set((x - 1, y - 1), FloorTile::BoxLeft);
                self.set((x, y), FloorTile::Nothing);
                self.set((x, y - 1), FloorTile::BoxRight);
            }

            (FloorTile::BoxLeft, Instruction::Down) => {
                self.push((x, y + 1), direction);
                self.push((x + 1, y + 1), direction);

                self.set((x, y), FloorTile::Nothing);
                self.set((x, y + 1), FloorTile::BoxLeft);
                self.set((x + 1, y), FloorTile::Nothing);
                self.set((x + 1, y + 1), FloorTile::BoxRight);
            }
            (FloorTile::BoxRight, Instruction::Down) => {
                self.push((x, y + 1), direction);
                self.push((x - 1, y + 1), direction);

                self.set((x - 1, y), FloorTile::Nothing);
                self.set((x - 1, y + 1), FloorTile::BoxLeft);
                self.set((x, y), FloorTile::Nothing);
                self.set((x, y + 1), FloorTile::BoxRight);
            }

            (FloorTile::Nothing, _) => {}
            (FloorTile::Wall, _) => {}
            _ => panic!(),
        }
    }

    fn move_check(&self, (x, y): Point, direction: Instruction) -> bool {
        let tile = self.get((x, y)).unwrap();
        match (tile, direction) {
            (FloorTile::BoxLeft, Instruction::Right) => self.move_check((x + 2, y), direction),
            (FloorTile::BoxRight, Instruction::Left) => self.move_check((x - 2, y), direction),

            (FloorTile::BoxLeft, Instruction::Upp) => {
                self.move_check((x, y - 1), direction) && self.move_check((x + 1, y - 1), direction)
            }
            (FloorTile::BoxRight, Instruction::Upp) => {
                self.move_check((x, y - 1), direction) && self.move_check((x - 1, y - 1), direction)
            }

            (FloorTile::BoxLeft, Instruction::Down) => {
                self.move_check((x, y + 1), direction) && self.move_check((x + 1, y + 1), direction)
            }
            (FloorTile::BoxRight, Instruction::Down) => {
                self.move_check((x, y + 1), direction) && self.move_check((x - 1, y + 1), direction)
            }

            (FloorTile::Nothing, _) => true,
            (FloorTile::Wall, _) => false,
            _ => panic!(),
        }
    }

    fn set(&mut self, (x, y): Point, tile: FloorTile) -> bool {
        if !self.in_bounds((x, y)) {
            return false;
        }
        self.floor[y as usize][x as usize] = tile;
        true
    }

    fn get(&self, (x, y): Point) -> Option<FloorTile> {
        if !self.in_bounds((x, y)) {
            return None;
        }
        Some(self.floor[y as usize][x as usize])
    }

    fn in_bounds(&self, (x, y): Point) -> bool {
        0 <= x && x < self.width && 0 <= y && y < self.height
    }

    fn total(&self) -> u32 {
        self.floor
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, tile)| {
                        if *tile == FloorTile::BoxLeft {
                            x as u32 + y as u32 * 100
                        } else {
                            0
                        }
                    })
                    .collect::<Vec<u32>>()
            })
            .sum()
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if x == self.robot.0 && y == self.robot.1 {
                    print!("@");
                } else {
                    print!("{}", self.get((x, y)).unwrap().to_char());
                }
            }
            println!();
        }

        println!();
    }
}

fn solve(input: &str) -> u32 {
    let (mut warehouse, instructions) = Warehouse::from_string(input);

    warehouse.execute_instructions(instructions);
    warehouse.print();

    let total: u32 = warehouse.total();

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_part2_main_example() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let output = solve(input);
        assert_eq!(output, 9021)
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_part2_example2() {
        let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        let output = solve(input);
        assert_eq!(output, 105)
    }
}
 */
