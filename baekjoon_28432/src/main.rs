use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let log_count = input.next().unwrap().trim().parse().unwrap();
    let mut log = Vec::with_capacity(log_count);
    let mut ask_index = 0;
    for i in 0..log_count {
        let ask = input.next().unwrap().trim().to_string();
        if ask == "?" {
            ask_index = i;
        }
        log.push(ask);
    }
    let last = if ask_index != 0 {
        let word: Vec<char> = log[ask_index - 1].chars().collect();
        word[word.len() - 1]
    } else {
        ' '
    };
    let first = if ask_index != log.len() - 1 {
        let word: Vec<char> = log[ask_index + 1].chars().collect();
        word[0]
    } else {
        ' '
    };

    let candidate_count = input.next().unwrap().trim().parse().unwrap();
    // let mut candidate = Vec::with_capacity(candidate_count);
    for _ in 0..candidate_count {
        let word = input.next().unwrap().trim().to_string();
        let word_char: Vec<char> = word.chars().collect();
        if (word_char[0] == last || last == ' ')
            && (word_char[word_char.len() - 1] == first || first == ' ')
            && !log.contains(&word)
        {
            stdout().write_all(format!("{word}").as_bytes()).unwrap();
        }
    }
}
