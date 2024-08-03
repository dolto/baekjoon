use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let mut writer = BufWriter::new(stdout().lock());

    let test_case_len = input.next().unwrap().parse().unwrap();
    for _ in 0..test_case_len {
        let x_size = input.next().unwrap().parse().unwrap();
        let y_size = input.next().unwrap().parse().unwrap();

        let mut farm = vec![vec![0; y_size]; x_size];

        let cabage_count = input.next().unwrap().parse().unwrap();
        for _ in 0..cabage_count {
            let x: usize = input.next().unwrap().parse().unwrap();
            let y: usize = input.next().unwrap().parse().unwrap();

            farm[x][y] = 1;
        }

        let mut result = 0;

        for x in 0..x_size {
            for y in 0..y_size {
                if farm[x][y] == 1 {
                    result += 1;
                    let mut bfs = vec![(x, y)];
                    while let Some((bx, by)) = bfs.pop() {
                        farm[bx][by] = 0;
                        if bx > 0 {
                            if farm[bx - 1][by] == 1 {
                                bfs.push((bx - 1, by));
                            }
                        }
                        if by > 0 {
                            if farm[bx][by - 1] == 1 {
                                bfs.push((bx, by - 1));
                            }
                        }
                        if bx + 1 < x_size {
                            if farm[bx + 1][by] == 1 {
                                bfs.push((bx + 1, by));
                            }
                        }
                        if by + 1 < y_size {
                            if farm[bx][by + 1] == 1 {
                                bfs.push((bx, by + 1));
                            }
                        }
                    }
                }
            }
        }

        writer.write_all(format!("{result}\n").as_bytes()).unwrap();
    }
    writer.flush().unwrap();
}
