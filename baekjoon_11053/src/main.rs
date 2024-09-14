use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let input = input
        .split_ascii_whitespace()
        .skip(1)
        .flat_map(|s| s.parse::<usize>())
        .collect::<Vec<usize>>();

    let mut solved = vec![0_usize; input.capacity()];

    for (index, &value) in input.iter().enumerate() {
        let min_next = value;
        let mut slen = 0;
        for i in (0..=index).rev() {
            // 현재 값보다 작으면서 개수는 더 큰 값을 찾음
            if min_next > input[i] && slen < solved[i] {
                slen = solved[i];
            }
        }
        // 자신이 수열에 포함되면서 가장 긴 값
        solved[index] = slen + 1;
    }

    let mut max = 0;
    for &len in solved.iter() {
        if max < len {
            max = len;
        }
    }

    stdout().write(max.to_string().as_bytes()).unwrap();
}
