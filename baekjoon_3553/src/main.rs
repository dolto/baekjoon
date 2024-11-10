use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    // 자릿수가 1000까지 있기 때문에, 일반적인 자료형으론 불가능하다.
    // d는 일반적인 자료형에 넣을 수 있으며, 3자릿수가 나오는 45로 나누어지는 수는 135, 4자릿수가 나오는 45로 나누어지는 수는 1350인 점을 미루어 보아, 뒤에 0을 붙이는 형식으로 구현할 수 있을 것 같다.

    let mut n = input.next().unwrap().parse::<usize>().unwrap();
    let d = input.next().unwrap();
    let mut bw = BufWriter::new(stdout().lock());

    if n > d.len() {
        let tn = i32::pow(10, d.len() as u32);
        n -= d.len() + 1;
        let mut d = d.parse::<i32>().unwrap();

        if tn % d == 0 {
            d *= tn / d;
        } else {
            d *= tn / d + 1;
        }

        bw.write_all(d.to_string().as_bytes()).unwrap();
        for _ in 0..n {
            bw.write("0".as_bytes()).unwrap();
        }
    } else if n == d.len() {
        bw.write(d.as_bytes()).unwrap();
    } else {
        bw.write("No solution".as_bytes()).unwrap();
    }

    bw.flush().unwrap();
}
