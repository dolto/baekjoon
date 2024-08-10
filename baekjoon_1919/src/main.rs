use std::{
    collections::HashMap,
    io::{stdin, stdout, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let left_word = input.next().unwrap();
    let mut left_hash = HashMap::with_capacity(left_word.chars().count());
    let mut left_word = left_word.chars();
    while let Some(ch) = left_word.next() {
        if let Some(w) = left_hash.get_mut(&ch) {
            *w += 1;
        } else {
            left_hash.insert(ch, 1);
        }
    }

    let right_word = input.next().unwrap();
    let mut right_hash = HashMap::with_capacity(right_word.chars().count());
    let mut right_word = right_word.chars();
    while let Some(ch) = right_word.next() {
        if let Some(w) = right_hash.get_mut(&ch) {
            *w += 1;
        } else {
            right_hash.insert(ch, 1);
        }
    }

    let mut count = 0;
    for (k, v) in left_hash.iter() {
        count += i32::abs(*v - *right_hash.get(k).unwrap_or(&0));
    }
    for (k, v) in right_hash.iter() {
        if left_hash.get(k).is_none() {
            count += *v;
        }
    }

    stdout().write_all(format!("{count}").as_bytes()).unwrap();
}
