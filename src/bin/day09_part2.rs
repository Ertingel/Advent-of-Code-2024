// cargo run  --bin day09_part2
// cargo test --bin day09_part2

fn main() {
    let input = include_str!("../././input/day09.txt");
    let output = solve(input);
    println!("Day09 part2: {output}");
}

#[derive(Debug, Clone)]
struct File {
    id: u64,
    space: std::ops::Range<u64>,
}

impl File {
    fn new(id: u64, space: std::ops::Range<u64>) -> File {
        File { id, space }
    }

    fn start(&self) -> u64 {
        self.space.start
    }

    fn end(&self) -> u64 {
        self.space.end
    }

    fn size(&self) -> u64 {
        self.end() - self.start()
    }

    fn move_to(&mut self, start: u64) -> &Self {
        let size = self.size();
        self.space.start = start;
        self.space.end = start + size;

        self
    }
}

fn parse_input(input: &str) -> Vec<File> {
    let mut out: Vec<File> = Vec::new();

    let mut index: u64 = 0;
    let mut id: u64 = 0;
    let mut iter = input.chars();

    while let Some(file_size) = iter.next() {
        let next_index = index + file_size.to_digit(10).unwrap() as u64;
        out.push(File::new(id, index..next_index));
        index = next_index;
        id += 1;

        if let Some(empty_length) = iter.next() {
            index += empty_length.to_digit(10).unwrap() as u64;
        }
    }

    out
}

fn compress_memory(files: &[File]) -> Vec<File> {
    let mut out: Vec<File> = Vec::from(files);

    for index1 in (1..out.len()).rev() {
        let file1 = &out[index1];
        let size = file1.size();

        for index2 in 0..(index1 - 1) {
            let file2_end = out[index2].end();
            let file3_start = out[index2 + 1].start();

            let space = file3_start - file2_end;

            if size <= space {
                let mut file1 = out.remove(index1);
                file1.move_to(file2_end);
                out.insert(index2 + 1, file1);

                break;
            }
        }

        //print_memory(&out);
    }

    out
}

/*
fn compress_memory(files: &[File]) -> Vec<File> {
    let mut remaining: VecDeque<File> = VecDeque::new();
    remaining.extend(files.iter().cloned());

    let mut out: Vec<File> = Vec::new();

    while !remaining.is_empty() {
        let f1 = remaining.pop_front();
        let Some(f1) = f1 else { break };

        let f2 = remaining.front();
        let Some(f2) = f2 else { break };

        let mut space = f2.space.start - f1.space.end;
        let mut start_index: u64 = f1.space.end;
        out.push(f1);

        println!("Keeping memory:");
        print_memory(&out);
        println!();

        for i in (0..remaining.len()).rev() {
            let f = remaining.get(i);
            let Some(f) = f else { break };

            let f_size = f.space.end - f.space.start;

            println!("Checking id {} size {}", f.id, f_size);

            if f_size <= space {
                let mut f = remaining.remove(i).unwrap();

                println!("Moving memory:");
                print_memory(&out);

                f.space.start = start_index;
                f.space.end = start_index + f_size;
                out.push(f);

                print_memory(&out);

                space -= f_size;
                start_index += f_size;

                println!("{space}");
            }

            if space == 0 {
                break;
            }
        }

        println!();
    }

    out
}
 */

/*
fn print_memory(files: &[File]) {
    let len = files.iter().map(|file| file.space.end).max().unwrap_or(0);
    for i in 0..len {
        let id = files
            .iter()
            .filter(|file| file.space.contains(&i))
            .map(|file| file.id)
            .next();

        if let Some(id) = id {
            print!("{id}");
        } else {
            print!(".");
        }
    }

    println!();
}
 */
fn compute_checksum(files: &[File]) -> u64 {
    files
        .iter()
        .map(|file| file.space.clone().map(|index| index * file.id).sum::<u64>())
        .sum()
}

fn solve(input: &str) -> u64 {
    let files = parse_input(input);
    let compressed = compress_memory(&files);
    let checksum: u64 = compute_checksum(&compressed);

    //println!("{:?}", files);
    //print_memory(&files);
    //print_memory(&compressed);
    //println!("{checksum}");

    /*
    2333133121414131402

    00...111...2...333.44.5555.6666.777.888899
    0099.111...2...333.44.5555.6666.777.8888..
    0099.1117772...333.44.5555.6666.....8888..
    0099.111777244.333....5555.6666.....8888..
    00992111777.44.333....5555.6666.....8888..
    */

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day09_part2() {
        let input = "2333133121414131402";
        let output = solve(input);
        assert_eq!(output, 2858)
    }
}
