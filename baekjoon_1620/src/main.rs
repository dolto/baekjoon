use std::{
    collections::HashMap,
    io::{stdin, stdout, BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(stdout().lock());

    let monster_count: usize = input.next().unwrap().parse().unwrap();
    let problum_count: usize = input.next().unwrap().parse().unwrap();

    let mut monster_vec = Vec::with_capacity(monster_count);
    let mut monster_hash = HashMap::with_capacity(monster_count);
    for i in 1..=monster_count {
        let monster_name = input.next().unwrap().to_string();
        monster_vec.push(monster_name.clone());
        monster_hash.insert(monster_name.clone(), i);
    }

    for _ in 0..problum_count {
        let problum = input.next().unwrap();
        if let Ok(p) = problum.parse::<usize>() {
            writer
                .write_all(format!("{}\n", monster_vec[p - 1]).as_bytes())
                .unwrap();
        } else {
            writer
                .write_all(
                    format!("{}\n", monster_hash.get(&problum.to_string()).unwrap()).as_bytes(),
                )
                .unwrap();
        }
    }
}
