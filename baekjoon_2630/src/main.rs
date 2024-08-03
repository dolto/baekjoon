use std::{
    io::{stdin, stdout, BufReader, Read, Write},
    ops::Add,
};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n = input.next().unwrap().parse().unwrap();
    let mut paper = vec![vec![0; n]; n];
    for x in 0..n {
        for y in 0..n {
            paper[x][y] = input.next().unwrap().parse().unwrap();
        }
    }

    let Paper(blue, white) = cutting(0, n, 0, n, &paper);
    stdout()
        .write_all(format!("{white}\n{blue}").as_bytes())
        .unwrap();
}

fn cutting(
    start_x: usize,
    end_x: usize,
    start_y: usize,
    end_y: usize,
    paper: &Vec<Vec<i32>>,
) -> Paper {
    let mut is_full = true;
    let color = paper[start_x][start_y];
    'check: for x in start_x..end_x {
        for y in start_y..end_y {
            if paper[x][y] != color {
                is_full = false;
                break 'check;
            }
        }
    }

    if is_full {
        if color == 0 {
            Paper(0, 1)
        } else {
            Paper(1, 0)
        }
    } else {
        let middle_x = (start_x + end_x) / 2;
        let middle_y = (start_y + end_y) / 2;
        let result = cutting(start_x, middle_x, start_y, middle_y, paper)
            + cutting(middle_x, end_x, start_y, middle_y, paper)
            + cutting(start_x, middle_x, middle_y, end_y, paper)
            + cutting(middle_x, end_x, middle_y, end_y, paper);
        // println!("{:?}", result);
        result
    }
}

#[derive(Debug)]
struct Paper(i32, i32);
impl Add for Paper {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Paper(self.0 + rhs.0, self.1 + rhs.1)
    }
}
