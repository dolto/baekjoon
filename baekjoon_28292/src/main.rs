use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<i32>().unwrap();

    // let mut p = "1".to_owned();
    // (1..input).for_each(|i| {
    //     let mut list = Vec::with_capacity(p.len());
    //     let mut pchras = p.chars();
    //     let mut count = 1;

    //     let mut before = pchras.next().unwrap();
    //     pchras.for_each(|p| {
    //         if before == p {
    //             count += 1;
    //         } else {
    //             list.push((before, count));
    //             count = 1;
    //         }
    //         before = p;
    //     });
    //     list.push((before, count));

    //     p.clear();
    //     list.into_iter().for_each(|(k, v)| {
    //         p.push(k);
    //         p.push_str(v.to_string().as_str());
    //     });

    //     let mut answer = '0';
    //     p.chars().for_each(|p| {
    //         if p > answer {
    //             answer = p;
    //         }
    //     });
    //     writeln!(stdout(), "{i}, {answer}").unwrap();
    // });

    if input <= 2 {
        stdout().write(1.to_string().as_bytes()).unwrap();
    } else if input <= 5 {
        stdout().write(2.to_string().as_bytes()).unwrap();
    } else {
        stdout().write(3.to_string().as_bytes()).unwrap();
    }
    stdout().flush().unwrap();
}
