use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(|s| s.parse::<f64>());

    let mut ps = Vec::with_capacity(input.next().unwrap() as usize);
    let qsize = input.next().unwrap() as usize;
    let mut min = 0.;

    for _ in 0..ps.capacity() {
        ps.push((input.next().unwrap(), input.next().unwrap()));
    }
    for _ in 0..qsize {
        let (qx, qy) = (input.next().unwrap(), input.next().unwrap());
        let mut max_distance = 0.;
        for &(px, py) in ps.iter() {
            let temp = ((px - qx).powi(2) + (py - qy).powi(2)).sqrt();
            max_distance = temp.max(max_distance);
        }
        min = (max_distance.powi(2).round()).max(min);
    }

    stdout()
        .write_all((min as usize).to_string().as_bytes())
        .unwrap();
}
