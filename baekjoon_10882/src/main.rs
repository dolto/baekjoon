use std::{
    fmt::Display,
    io::{stdin, stdout, BufWriter, Read, Write},
    ops::{Add, Sub},
};

#[derive(Clone, Copy, PartialEq)]
struct UtcClock {
    houre: i32,
    minit: f32,
}
impl UtcClock {
    fn from_base_time(base_time: &str) -> Self {
        let mut base_time = base_time.split("UTC").skip(1).next().unwrap().split(".");

        let h = base_time.next().unwrap().parse::<i32>().unwrap();
        let m = base_time.next().unwrap_or("0");
        let m = (60. / f32::powi(10., m.len() as i32)) * m.parse::<f32>().unwrap();

        Self { houre: h, minit: m }
    }
    fn from_time(time: &str) -> Self {
        let mut time = time.split(":");
        let houre = time.next().unwrap().parse().unwrap();
        let minit = time.next().unwrap().parse().unwrap();
        Self { houre, minit }
    }
}

impl Display for UtcClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.houre, self.minit)
    }
}
impl Add for UtcClock {
    type Output = UtcClock;
    fn add(self, rhs: Self) -> Self::Output {
        let mut h = self.houre + rhs.houre;
        let mut m = self.minit + rhs.minit;

        if m >= 60. {
            m -= 60.;
            h += 1;
        } else if m < 0. {
            m += 60.;
            h -= 1;
        }

        if h >= 24 {
            h -= 24;
        } else if h < 0 {
            h += 24;
        }

        Self { houre: h, minit: m }
    }
}
impl Sub for UtcClock {
    type Output = UtcClock;
    fn sub(self, rhs: Self) -> Self::Output {
        self + Self {
            houre: -rhs.houre,
            minit: -rhs.minit,
        }
    }
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().skip(1);

    let yj_base = UtcClock::from_time(input.next().unwrap())
        - UtcClock::from_base_time(input.next().unwrap());

    let mut bw = BufWriter::new(stdout().lock());
    while let Some(time) = input.next() {
        writeln!(bw, "{}", (yj_base + UtcClock::from_base_time(time))).unwrap();
    }

    // for i in -12..=12 {
    //     for j in [0, 5] {
    //         let time = format!("UTC{}.{}", i, j);
    //         writeln!(
    //             bw,
    //             "{}",
    //             (yj_base + UtcClock::from_base_time(time.as_str()))
    //         )
    //         .unwrap();
    //     }
    // }

    bw.flush().unwrap();
}
