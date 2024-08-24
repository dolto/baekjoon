use std::{
    collections::VecDeque,
    io::{stdin, stdout, BufWriter, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(stdout().lock());

    while let Some(s) = input.next() {
        if s == "#" {
            break;
        } else {
            let mut chs: VecDeque<char> = s.chars().collect();
            let mut count = chs.len();
            while let Some(ch) = chs.pop_front() {
                match ch {
                    'a' | 'e' | 'i' | 'o' | 'u' => {
                        chs.push_front(ch);
                        chs.push_back('a');
                        chs.push_back('y');
                        break;
                    }
                    _ => {
                        chs.push_back(ch);
                    }
                }
                count -= 1;
                if count == 0 {
                    chs.push_back('a');
                    chs.push_back('y');
                    break;
                }
            }
            writer
                .write_all(format!("{}\n", chs.iter().collect::<String>()).as_bytes())
                .unwrap();
        }
    }

    writer.flush().unwrap();
}
