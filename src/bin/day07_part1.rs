// cargo run  --bin day07_part1
// cargo test --bin day07_part1

fn main() {
    let input = include_str!("../././input/day07.txt");
    let output = solve(input);
    println!("Day07 part1: {output}");
}

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            let mut res = line.split(": ");
            let result: i64 = res.next().unwrap().parse().unwrap();

            let numbers: Vec<i64> = res
                .next()
                .unwrap()
                .split(' ')
                .map(|n| n.parse::<i64>().unwrap())
                .collect();

            (result, numbers)
        })
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Operator {
    Addition,
    Multiplication,
}

impl Operator {
    fn operate(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Addition => a + b,
            Operator::Multiplication => a * b,
        }
    }

    fn types() -> std::array::IntoIter<Operator, 2> {
        [Operator::Addition, Operator::Multiplication].into_iter()
    }
}

fn find_solution(
    result: &i64,
    numbers: Vec<i64>,
    acc: &i64,
    stack: Vec<Operator>,
) -> Option<Vec<Operator>> {
    if numbers.is_empty() {
        if result == acc {
            return Some(stack.clone());
        } else {
            return None;
        }
    }

    for operator in Operator::types() {
        let mut numbers2 = numbers.clone();
        let acc2 = operator.operate(*acc, numbers2.remove(0));
        let mut stack2 = stack.clone();
        stack2.push(operator);

        let res = find_solution(result, numbers2, &acc2, stack2);
        if let Some(res) = res {
            return Some(res);
        }
    }

    None
}

fn solve_equations(equations: &[(i64, Vec<i64>)]) -> Vec<(i64, Vec<i64>, Option<Vec<Operator>>)> {
    equations
        .iter()
        .map(|(result, numbers)| {
            (
                *result,
                numbers.clone(),
                find_solution(result, numbers.clone(), &0, Vec::new()),
            )
        })
        .collect()
}

fn solve(input: &str) -> i64 {
    let equations: Vec<(i64, Vec<i64>)> = parse_input(input);

    let solved = solve_equations(&equations);

    let valid: Vec<(i64, Vec<i64>, Vec<Operator>)> = solved
        .clone()
        .into_iter()
        .filter_map(|(result, numbers, solution)| {
            if let Some(solution) = solution {
                return Some((result, numbers, solution));
            }
            None
        })
        .collect();

    let total: i64 = valid.clone().into_iter().map(|(result, _, _)| result).sum();

    /*
    println!("{:?}", equations);
    println!("{:?}", solved);
    println!("{:?}", valid);
    println!("{total}");
    */

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day07_part1() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let output = solve(input);
        assert_eq!(output, 3749)
    }
}
