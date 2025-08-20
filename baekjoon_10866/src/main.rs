use std::{
    collections::VecDeque,
    io::{BufReader, BufWriter, Read, Write, stdin, stdout},
};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    reader.read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let n = input.next().unwrap().parse::<usize>().unwrap();

    let mut deque = VecDeque::with_capacity(n);

    input.for_each(|s| {
        let mut input = s.split_ascii_whitespace();
        let commend = input.next().unwrap();

        match commend {
            "push_back" => {
                deque.push_back(input.next().unwrap().parse::<i32>().unwrap());
            }
            "push_front" => {
                deque.push_front(input.next().unwrap().parse::<i32>().unwrap());
            }
            "pop_back" => {
                if let Some(p) = deque.pop_back() {
                    writeln!(writer, "{}", p).unwrap();
                } else {
                    writeln!(writer, "-1").unwrap();
                }
            }
            "pop_front" => {
                if let Some(p) = deque.pop_front() {
                    writeln!(writer, "{}", p).unwrap();
                } else {
                    writeln!(writer, "-1").unwrap();
                }
            }
            "size" => {
                writeln!(writer, "{}", deque.len()).unwrap();
            }
            "empty" => {
                if deque.is_empty() {
                    writeln!(writer, "1").unwrap();
                } else {
                    writeln!(writer, "0").unwrap();
                }
            }
            "front" => {
                if let Some(p) = deque.front() {
                    writeln!(writer, "{}", p).unwrap();
                } else {
                    writeln!(writer, "-1").unwrap();
                }
            }
            "back" => {
                if let Some(p) = deque.back() {
                    writeln!(writer, "{}", p).unwrap();
                } else {
                    writeln!(writer, "-1").unwrap();
                }
            }
            _ => {}
        }
    });

    writer.flush().unwrap();
}
