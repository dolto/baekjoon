use std::{
    collections::HashMap,
    io::{stdin, stdout, BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(stdout().lock());

    let test_case_count = input.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..test_case_count {
        let wear_count = input.next().unwrap().parse::<usize>().unwrap();
        let mut hashmap = HashMap::with_capacity(wear_count);
        for _ in 0..wear_count {
            input.next();
            let category = input.next().unwrap().to_owned();
            if let Some(name) = hashmap.get_mut(&category) {
                *name += 1
            } else {
                hashmap.insert(category, 2_i64);
            }
        }

        // let vec = hashmap.iter().map(|(_, v)| *v).collect::<Vec<i32>>();
        // let mut result = 0;
        // if vec.len() == 1 {
        //     ncr(&vec, Vec::with_capacity(1), 1, 0, &mut result);
        // } else {
        //     for i in 1..=vec.len() {
        //         ncr(&vec, Vec::with_capacity(i), i, 0, &mut result);
        //     }
        // }

        let mut result = 1;
        for (_, &i) in hashmap.iter() {
            result *= i;
        }
        result -= 1;

        writer
            .write_all(format!("{}\n", result).as_bytes())
            .unwrap();
    }
    writer.flush().unwrap();
}

// req: 요소들
// res: 현재까지 뽑은 요소
// r: 남은 요소 개수
// depth: 지금까지 확인한 인덱스
// fn ncr(req: &Vec<i32>, mut res: Vec<i32>, r: usize, depth: usize, result: &mut i32) {
//     if r == 0 {
//         let mut temp = 0;
//         while let Some(i) = res.pop() {
//             if temp != 0 {
//                 temp *= i;
//             } else {
//                 temp = i;
//             }
//         }
//         *result += temp;
//     } else if depth != req.len() {
//         ncr(req, res.clone(), r, depth + 1, result);
//         res.push(req[depth]);
//         ncr(req, res, r - 1, depth + 1, result);
//     }
// }
