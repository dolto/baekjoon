use std::{
    collections::HashMap,
    io::{stdin, stdout, BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let test_case = input.next().unwrap();
    let mut bw = BufWriter::new(stdout().lock());
    for case in 1..=test_case {
        let length = input.next().unwrap();
        let mut log = Vec::with_capacity(length as usize);
        for _ in 0..length {
            log.push(input.next().unwrap());
        }

        let mut is_anti = false;
        'label: for i in 0..length as usize - 1 {
            let mut corrents = HashMap::with_capacity(length as usize);
            let mut temp = HashMap::with_capacity(length as usize);
            for j in i + 1..length as usize {
                corrents.entry(log[i] - log[j]).or_insert(log[i]);
                // println!("c: {:?}", corrents);
                for (k, v) in &mut corrents {
                    if *k == *v - log[j] {
                        *temp.entry(*v - log[j]).or_insert(1) += 1;
                        *v = log[j];
                    }
                }
                for (_k, v) in &temp {
                    if *v >= 3 {
                        // println!("{} {}", _k, v);
                        is_anti = true;
                        break 'label;
                    }
                }
            }
        }
        writeln!(bw, "Case #{case}: {}", if is_anti { "NO" } else { "YES" }).unwrap();
    }

    bw.flush().unwrap();
}
