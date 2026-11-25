use std::io::{BufReader, BufWriter, Read, Write, stdin, stdout};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    reader.read_to_string(&mut input).unwrap();
    let mut i = input
        .split_ascii_whitespace()
        .flat_map(|i| i.parse::<i32>());

    let t = i.next().unwrap();

    for idx in (0..t).rev() {
        let (x1, y1, r1, x2, y2, r2) = (
            i.next().unwrap(),
            i.next().unwrap(),
            i.next().unwrap(),
            i.next().unwrap(),
            i.next().unwrap(),
            i.next().unwrap(),
        );

        // 두 터렛의 거리를 구함
        let d_sq = (x2 - x1).pow(2) + (y2 - y1).pow(2);

        let r_msq = (r1 - r2).pow(2);
        let r_psq = (r1 + r2).pow(2);
        let res = if d_sq == 0 && r1 == r2 {
            -1
        } else if r_msq < d_sq && d_sq < r_psq {
            2
        } else if r_msq == d_sq || r_psq == d_sq {
            1
        } else {
            0
        };

        let res = res.to_string();
        if 0 < idx {
            writer.write(format!("{}\n", res).as_bytes()).unwrap();
        } else {
            writer.write(format!("{}", res).as_bytes()).unwrap();
        }
    }

    writer.flush().unwrap();
}
