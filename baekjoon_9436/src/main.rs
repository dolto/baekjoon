use std::{
    collections::VecDeque,
    io::{stdin, stdout, BufWriter, Read, StdoutLock, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(|s| s.parse());
    let mut bw = BufWriter::new(stdout().lock());

    while let (Some(n), Some(t)) = (input.next(), input.next()) {
        solved(n, t, &mut bw);
    }

    // for i in 2..=100 {
    //     for j in 2..=100 {
    //         solved(i, j, &mut bw);
    //     }
    // }

    bw.flush().unwrap();
}

fn solved(n: usize, t: usize, bw: &mut BufWriter<StdoutLock<'static>>) {
    let mut ns = VecDeque::from_iter(vec![0; n]);
    loop {
        for _ in 0..t {
            ns[0] += 1;
            ns.rotate_right(1);
        }
        ns.rotate_left(1);
        ns.pop_front();
        ns.rotate_right(1);
        // writeln!(bw, "{:?}", ns).unwrap();

        let mut b = true;
        for &i in ns.iter() {
            if i != ns[0] {
                b = false;
                break;
            }
        }
        if b {
            break;
        }
    }
    writeln!(bw, "{} {}", ns.len(), ns[0]).unwrap();
}
