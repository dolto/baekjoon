use std::{
    collections::HashMap,
    io::{stdin, stdout, BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(|s| s.parse());
    let test_case = input.next().unwrap();
    let mut candidate = Vec::with_capacity(test_case);
    while let Some(candi) = input.next() {
        candidate.push(candi);
    }

    let max_candi = *candidate.iter().max().unwrap() + 1;
    let mut eratos = vec![true; max_candi];
    eratos[0] = false;
    eratos[1] = false;

    let mut count = 2;
    while max_candi >= usize::pow(count, 2) {
        if eratos[count] {
            step_set(count, &mut eratos, max_candi);
        }
        count += 1;
    }
    let eratos: Vec<usize> = eratos
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| if b { Some(i) } else { None })
        .collect();

    // println!("{:?}", eratos);

    let mut bw = BufWriter::new(stdout().lock());
    for c in candidate {
        let mut c = c;
        let mut answer = HashMap::with_capacity(max_candi);
        while c != 1 {
            for &era in eratos.iter() {
                if c % era == 0 {
                    *answer.entry(era).or_insert(0) += 1;
                    c /= era;
                    break;
                }
            }
        }

        let mut answer: Vec<(usize, i32)> = answer.into_iter().collect();
        answer.sort_by(|a, b| a.0.cmp(&b.0));
        answer.into_iter().for_each(|(i, v)| {
            writeln!(bw, "{} {}", i, v).unwrap();
        })
    }

    bw.flush().unwrap();
}

fn step_set(step: usize, eratos: &mut Vec<bool>, max_candi: usize) {
    for i in ((step * 2)..max_candi).step_by(step) {
        eratos[i] = false;
    }
}
