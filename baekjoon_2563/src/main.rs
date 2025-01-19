use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let mut place = [[true; 100]; 100];
    let mut answer = 0;

    (0..input.next().unwrap()).for_each(|_| {
        let (x, y) = (input.next().unwrap() - 1, input.next().unwrap() - 1);
        (y..y + 10).for_each(|i| {
            (x..x + 10).for_each(|j| {
                if place[i][j] {
                    answer += 1;
                    place[i][j] = false;
                }
            })
        });
    });

    stdout().write(answer.to_string().as_bytes()).unwrap();
    stdout().flush().unwrap();
}
