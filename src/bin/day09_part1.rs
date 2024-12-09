// cargo run  --bin day09_part1
// cargo test --bin day09_part1

fn main() {
    let input = include_str!("../././input/day09.txt");
    let output = solve(input);
    println!("Day09 part1: {output}");
}

fn parse_input(input: &str) -> Vec<Option<u32>> {
    let mut out: Vec<Option<u32>> = Vec::new();

    let mut id: u32 = 0;
    let mut iter = input.chars();

    while let Some(file_size) = iter.next() {
        let file_size = file_size.to_digit(10).unwrap();
        for _ in 0..file_size {
            out.push(Some(id));
        }
        id += 1;

        if let Some(empty_length) = iter.next() {
            let empty_length = empty_length.to_digit(10).unwrap();
            for _ in 0..empty_length {
                out.push(None);
            }
        }
    }

    out
}

fn compress_memory(files: &[Option<u32>]) -> Vec<u32> {
    let mut out: Vec<u32> = Vec::new();
    let mut end = files.len();

    for index in 0..files.len() {
        if index >= end {
            return out;
        }

        if let Some(id) = files[index] {
            out.push(id);
            continue;
        }

        while index < end {
            end -= 1;
            if let Some(id) = files[end] {
                out.push(id);
                break;
            }
        }
    }

    out
}

fn compute_checksum(files: &[u32]) -> u64 {
    files
        .iter()
        .enumerate()
        .map(|(index, id)| {
            let a = index as u64;
            let b = *id as u64;
            a * b
        })
        .sum()
}
/*
fn print_memory(files: &[Option<u32>]) {
    for id in files {
        if let Some(id) = id {
            print!("{id}");
        } else {
            print!(".");
        }
    }
    println!();
}

fn print_memory_compressed(files: &[u32]) {
    for id in files {
        print!("{id}");
    }
    println!();
}
 */
fn solve(input: &str) -> u64 {
    let files = parse_input(input);
    let compressed = compress_memory(&files);
    let checksum: u64 = compute_checksum(&compressed);

    //println!("{:?}", files);
    //print_memory(&files);
    //print_memory_compressed(&compressed);
    //println!("{checksum}");

    /*
    2333133121414131402

    00...111...2...333.44.5555.6666.777.888899
    009..111...2...333.44.5555.6666.777.88889.
    0099.111...2...333.44.5555.6666.777.8888..
    00998111...2...333.44.5555.6666.777.888...
    009981118..2...333.44.5555.6666.777.88....
    0099811188.2...333.44.5555.6666.777.8.....
    009981118882...333.44.5555.6666.777.......
    0099811188827..333.44.5555.6666.77........
    00998111888277.333.44.5555.6666.7.........
    009981118882777333.44.5555.6666...........
    009981118882777333644.5555.666............
    00998111888277733364465555.66.............
    0099811188827773336446555566..............
    */

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day09_part1() {
        let input = "2333133121414131402";
        let output = solve(input);
        assert_eq!(output, 1928)
    }
}
