use std::{
    collections::VecDeque,
    io::{stdin, stdout, BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<i32>());
    let mut writer = BufWriter::new(stdout().lock());

    let x = input.next().unwrap() as usize;
    let y = input.next().unwrap() as usize;
    let mut map = vec![vec![-1; y]; x];
    let mut memory = VecDeque::with_capacity(x * y);

    for x in 0..x {
        for y in 0..y {
            let temp = input.next().unwrap();
            if temp == 2 {
                map[x][y] = 0;
                if x > 0 {
                    memory.push_back((x - 1, y, 1));
                }
                if x < map.len() - 1 {
                    memory.push_back((x + 1, y, 1));
                }
                if y > 0 {
                    memory.push_back((x, y - 1, 1));
                }
                if y < map[x].len() - 1 {
                    memory.push_back((x, y + 1, 1));
                }
            } else if temp == 0 {
                map[x][y] = 0;
            }
        }
    }

    while let Some((x, y, z)) = memory.pop_front() {
        if map[x][y] != -1 {
            continue;
        }
        map[x][y] = z;
        if x > 0 {
            memory.push_back((x - 1, y, z + 1));
        }
        if x < map.len() - 1 {
            memory.push_back((x + 1, y, z + 1));
        }
        if y < map[x].len() - 1 {
            memory.push_back((x, y + 1, z + 1));
        }
        if y > 0 {
            memory.push_back((x, y - 1, z + 1));
        }
    }

    for x in 0..x {
        for y in 0..y {
            writer
                .write_all(format!("{} ", map[x][y]).as_bytes())
                .unwrap();
        }
        writer.write_all(format!("\n").as_bytes()).unwrap();
    }
    writer.flush().unwrap();
}
