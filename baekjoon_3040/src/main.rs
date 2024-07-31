use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    reader.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let mut candidate = Vec::with_capacity(9);

    let mut total = 0;
    for _ in 0..9 {
        let n = input.next().unwrap().parse::<i32>().unwrap();
        total += n;
        candidate.push(n);
    }

    let fakers = total - 100;
    let mut f1 = 0;
    let mut f2 = 0;

    'serche: for i in 0..9 {
        for j in (i + 1)..9 {
            if candidate[i] + candidate[j] == fakers {
                f1 = i;
                f2 = j;
                break 'serche;
            }
        }
    }
    candidate.remove(f1);
    candidate.remove(f2 - 1);

    for i in 0..7 {
        writer
            .write_all(format!("{}\n", candidate[i]).as_bytes())
            .unwrap();
    }
}
