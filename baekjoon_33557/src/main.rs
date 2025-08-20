use std::{
    collections::VecDeque,
    io::{Read, Write, stdin, stdout},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    (0..input.next().unwrap().parse::<i32>().unwrap()).for_each(|_| {
        let (a, b) = (input.next().unwrap(), input.next().unwrap());

        let multifly = a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap();
        let mut vec = VecDeque::new();

        let mut a = a.bytes().rev();
        let mut b = b.bytes().rev();
        loop {
            let oa = a.next();
            let ob = b.next();

            if oa.is_none() && ob.is_none() {
                break;
            }
            let oa = oa.unwrap_or(b'1') - b'0';
            let ob = ob.unwrap_or(b'1') - b'0';

            vec.push_front(oa as i64 * ob as i64);
        }

        let s = vec.iter().map(|i| i.to_string()).collect::<String>();

        if s == multifly.to_string() {
            writeln!(stdout(), "1").unwrap();
        } else {
            writeln!(stdout(), "0").unwrap();
        }

        stdout().flush().unwrap();
    });
}
