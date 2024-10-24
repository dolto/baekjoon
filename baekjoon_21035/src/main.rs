use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let y = input.next().unwrap().parse().unwrap();
    let x = input.next().unwrap().parse().unwrap();
    let mut map = Vec::with_capacity(y);

    let mut min = i32::MAX;
    let mut max = i32::MIN;
    let mut sum = 0;
    for _ in 0..y {
        let mut temp = Vec::with_capacity(x);
        for _ in 0..x {
            let val = input.next().unwrap().parse::<i32>().unwrap();
            temp.push(val);
            sum += val;
        }
        map.push(temp);
    }

    let query_size = input.next().unwrap().parse().unwrap();
    for _ in 0..query_size {
        if input.next().unwrap() == "row" {
            let y = input.next().unwrap().parse::<usize>().unwrap() - 1;
            let val = input.next().unwrap().parse::<i32>().unwrap();
            sum += x as i32 * val;
            for x in 0..x {
                map[y][x] += val;
            }
        } else {
            let x = input.next().unwrap().parse::<usize>().unwrap() - 1;
            let val = input.next().unwrap().parse::<i32>().unwrap();
            sum += y as i32 * val;
            for y in 0..y {
                map[y][x] += val;
            }
        }
    }

    for x in 0..x {
        for y in 0..y {
            min = min.min(map[y][x]);
            max = max.max(map[y][x]);
        }
    }

    stdout()
        .write_all(format!("{} {} {}", sum, min, max).as_bytes())
        .unwrap();
}
