use std::io::{BufWriter, Read, Write, stdin, stdout};

fn main() {
    let mut buffer = String::new();
    let mut writer = BufWriter::new(stdout().lock());
    stdin().read_to_string(&mut buffer).unwrap();

    let mut input = buffer
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<i32>());

    let n = input.next().unwrap();
    let mut m = input.next().unwrap();
    let mm = m;
    let bm = input.next().unwrap();
    let t = input.next().unwrap();
    let r = input.next().unwrap();

    let mut tn = 0;
    let mut answer = 0;

    if bm - mm < t {
        answer = -1;
    } else {
        loop {
            // (100 - 50) / 5 = 10
            // (200 - 50) / 150 = 1
            // 50 + 150 = 200
            let ex_count = (bm - m) / t;
            if ex_count > 0 {
                // 운동을 ex_count분 만큼 할 수 있음
                if tn + ex_count >= n {
                    answer += ex_count.min(n - tn);
                    break;
                } else {
                    m += ex_count * t;
                    tn += ex_count;
                    answer += ex_count;
                }
            } else {
                // r - 1을 더하는 이유는 올림연산을 하기 위해서
                let re_count = (m - (bm - t) + r - 1) / r;
                answer += re_count;
                m = (m - re_count * r).max(mm);
            }
        }
    }

    write!(writer, "{}", answer).unwrap();
    writer.flush().unwrap();
}
