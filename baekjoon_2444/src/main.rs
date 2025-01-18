use std::io::{stdin, stdout, BufWriter, Write};
const STAR: u8 = '*' as u8;
const EMPTY: u8 = ' ' as u8;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<usize>().unwrap();

    let size = (input * 2).checked_sub(1).unwrap_or(0);
    let mut stars = vec![EMPTY; size];
    let mid = size / 2;
    let mut bw = BufWriter::new(stdout().lock());
    for i in 0..=mid {
        let arr = [mid - i, mid + i];
        for j in arr {
            stars[j] = STAR;
        }
        bw.write(&stars[..mid + i + 1]).unwrap();
        bw.write("\n".as_bytes()).unwrap();
    }
    for i in (1..=mid).rev() {
        let arr = [mid - i, mid + i];
        for j in arr {
            stars[j] = EMPTY;
        }
        bw.write(&stars[..mid + i]).unwrap();
        bw.write("\n".as_bytes()).unwrap();
    }
    bw.flush().unwrap();
}
