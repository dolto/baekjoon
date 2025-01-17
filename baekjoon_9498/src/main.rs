use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<i32>().unwrap();

    let mut hashmap = HashMap::with_capacity(4);
    hashmap.insert(90, "A");
    hashmap.insert(80, "B");
    hashmap.insert(70, "C");
    hashmap.insert(60, "D");
    for cut in (0..=90).rev().step_by(10) {
        if input >= cut {
            stdout()
                .write(hashmap.get(&cut).unwrap_or(&"F").as_bytes())
                .unwrap();
            break;
        }
    }
}
