use std::{
    collections::VecDeque,
    io::{stdin, stdout, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.trim().split("\n").skip(1);
    let mut main_dack = VecDeque::with_capacity(27);
    // 조커는 true
    main_dack.push_back(false);
    for _ in 0..26 {
        main_dack.push_back(true);
    }

    while let Some(sh) = input.next() {
        let mut input = sh.split_ascii_whitespace().flat_map(|s| s.parse::<usize>());
        let mut ldack = VecDeque::with_capacity(13);
        let mut rdack = VecDeque::with_capacity(14);
        for _ in 0..13 {
            ldack.push_back(main_dack.pop_front().unwrap());
        }
        for _ in 0..14 {
            rdack.push_back(main_dack.pop_front().unwrap());
        }

        let mut is_r = true;
        while let Some(size) = input.next() {
            if is_r {
                for _ in 0..size {
                    main_dack.push_back(rdack.pop_front().unwrap());
                }
            } else {
                for _ in 0..size {
                    main_dack.push_back(ldack.pop_front().unwrap());
                }
            }
            is_r = !is_r;
        }
    }

    let mut index = 0;
    while main_dack[index] {
        index += 1;
    }
    stdout().write((index + 1).to_string().as_bytes()).unwrap();
}
