use std::{
    io::{stdin, stdout, Read, Write},
    ops::Mul,
};

#[derive(PartialEq, Clone, Copy)]
struct Of(f64);
impl Eq for Of {}
impl PartialOrd for Of {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
impl Ord for Of {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}
impl Mul for Of {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Of(self.0 * rhs.0)
    }
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<f64>());

    let count = input.next().unwrap() as usize;
    let frist = input.next().unwrap();
    let mut a = Vec::with_capacity(count);
    a.push(Of(frist));
    let mut d = Vec::with_capacity(count);
    d.push(Of(frist));

    let mut max = frist;
    for i in 1..count {
        let present = Of(input.next().unwrap());
        a.push(present);
        d.push(present.max(present * d[i - 1]));
        max = max.max(d[i].0);
    }

    max = max * 1000.;
    max = max.round();
    max = max * 0.001;
    stdout()
        .write_all(format!("{:.3}", max).as_bytes())
        .unwrap();
}
