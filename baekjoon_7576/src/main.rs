use std::{
    collections::VecDeque,
    io::{stdin, stdout, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<i32>());

    let x = input.next().unwrap() as usize;
    let y = input.next().unwrap() as usize;
    // println!("{x} {y}");

    let mut map = vec![vec![-1; y]; x];
    let mut started = VecDeque::with_capacity(x * y);
    let mut after = VecDeque::with_capacity(x * y);
    let mut count = 0;
    let mut tcount = 0;
    for y in 0..y {
        for x in 0..x {
            let temp = input.next().unwrap();
            map[x][y] = temp;
            if temp == 1 {
                started.push_back((x, y));
            } else if temp == 0 {
                tcount += 1;
            }
        }
    }

    // println!("{tcount} {count}");
    while !started.is_empty() || !after.is_empty() {
        if after.is_empty() {
            while let Some((x, y)) = started.pop_front() {
                spread(x, y, &mut tcount, &mut after, &mut map);
            }
            let mut check = false;
            for (x, y) in after.iter() {
                if map[*x][*y] != -1 {
                    check = true;
                    break;
                }
            }
            if check {
                count += 1;
            }
        } else if started.is_empty() {
            while let Some((x, y)) = after.pop_front() {
                spread(x, y, &mut tcount, &mut started, &mut map);
            }
            let mut check = false;
            for (x, y) in started.iter() {
                if map[*x][*y] != -1 {
                    check = true;
                    break;
                }
            }
            if check {
                count += 1;
            }
        }

        // for y in 0..y {
        //     for x in 0..x {
        //         print!("{} ", map[x][y]);
        //     }
        //     println!();
        // }
        // println!();
    }

    // println!("{tcount} {count}");
    if tcount == 0 {
        stdout().write_all(format!("{count}").as_bytes()).unwrap();
    } else {
        stdout().write_all("-1".as_bytes()).unwrap();
    }
}

fn spread(
    x: usize,
    y: usize,
    tcount: &mut i32,
    nexts: &mut VecDeque<(usize, usize)>,
    map: &mut Vec<Vec<i32>>,
) {
    if map[x][y] != -1 {
        if map[x][y] == 0 {
            *tcount -= 1;
        }
        map[x][y] = -1;
        if x > 0 && map[x - 1][y] == 0 {
            nexts.push_back((x - 1, y));
        }
        if y > 0 && map[x][y - 1] == 0 {
            nexts.push_back((x, y - 1));
        }
        if x < map.len() - 1 && map[x + 1][y] == 0 {
            nexts.push_back((x + 1, y));
        }
        if y < map[x].len() - 1 && map[x][y + 1] == 0 {
            nexts.push_back((x, y + 1));
        }
    }
}
