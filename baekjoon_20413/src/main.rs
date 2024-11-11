use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().skip(1);

    let (s, g, p, d): (i32, i32, i32, i32) = (
        input.next().unwrap().parse().unwrap(),
        input.next().unwrap().parse().unwrap(),
        input.next().unwrap().parse().unwrap(),
        input.next().unwrap().parse().unwrap(),
    );

    let m = input.next().unwrap().chars();
    let mut answer = 0;
    let mut grid = [0, 0];
    let mut count = 0;

    for m in m {
        let temp;
        grid[count] = 0;
        let sum = grid.iter().sum::<i32>();
        match m {
            'B' => {
                temp = s;
            }
            'S' => {
                temp = g;
            }
            'G' => {
                temp = p;
            }
            'P' => {
                temp = d;
            }
            'D' => {
                temp = d;
            }
            _ => {
                temp = 0;
            }
        }

        if m != 'D' {
            grid[count] = temp - sum - 1;
        } else {
            grid[count] = temp;
        }
        answer += grid[count];

        count = (count + 1) % 2;
    }

    stdout().write(answer.to_string().as_bytes()).unwrap();
}
