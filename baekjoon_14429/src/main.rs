use std::{
    cmp::Ordering,
    fmt::Display,
    io::{stdin, stdout, Read, Write},
};

#[derive(PartialEq, Eq)]
struct Game {
    number: usize,
    round: usize,
}
impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Game {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.round > other.round {
            Ordering::Greater
        } else if self.round < other.round {
            Ordering::Less
        } else if self.number > other.number {
            Ordering::Greater
        } else if self.number < other.number {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}
impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.number, self.round)
    }
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(|s| s.parse());

    let count = input.next().unwrap();
    let mut min_list = Vec::with_capacity(count);

    for i in 1..=count {
        let mut j = input.next().unwrap();
        let m = input.next().unwrap();
        let step = m + 1;
        j -= (j - 1) % step;

        let res = 2 * (j / m + 1);
        min_list.push(Game {
            number: i,
            round: res,
        });
    }

    min_list.sort();
    let min = &min_list[0];
    // let mut writer = BufWriter::new(stdout().lock());
    // min_list
    //     .iter()
    //     .filter(|g| g.round == min)
    //     .for_each(|f| write!(writer, "{} ", f).unwrap());
    write!(stdout(), "{}", min).unwrap();
    // writer.flush().unwrap();
}
