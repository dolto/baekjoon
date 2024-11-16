use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<usize>());

    let table = make_pascal(30);
    let (r, c, w) = (
        input.next().unwrap() - 1,
        input.next().unwrap() - 1,
        input.next().unwrap(),
    );

    let mut answer = 0;
    for i in 0..w {
        for j in 0..i + 1 {
            answer += table[r + i][c + j];
        }
    }

    print!("{answer}");
}

fn make_pascal(size: usize) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::with_capacity(size);

    for i in 1..=size {
        let mut temp = Vec::with_capacity(i);
        if let Some(ui) = i.checked_sub(2) {
            for j in 0..i {
                let left = if let Some(li) = j.checked_sub(1) {
                    result[ui][li]
                } else {
                    0
                };
                let &right = result[ui].get(j).unwrap_or(&0);
                temp.push(left + right);
            }
        } else {
            temp.push(1);
        }

        result.push(temp);
    }
    result
}
