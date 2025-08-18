use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    io::{BufReader, BufWriter, Read, Write, stdin, stdout},
};

#[derive(Clone)]
struct City {
    bus_index: Option<Vec<Bus>>,
    cost: i32,
}

#[derive(PartialEq, Eq)]
struct CityIndex {
    cost: i32,
    index: usize,
}
impl PartialOrd for CityIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CityIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.cost > other.cost {
            Ordering::Greater
        } else if self.cost < other.cost {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

#[derive(Clone, Copy)]
struct Bus {
    cost: i32,
    city_index: usize,
}

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    reader.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let city_count = input.next().unwrap().parse::<usize>().unwrap();
    let bus_count = input.next().unwrap().parse::<usize>().unwrap();

    let mut citys = vec![
        City {
            cost: i32::MAX,
            bus_index: Some(Vec::new())
        };
        city_count
    ];

    for _ in 0..bus_count {
        let start_city = input.next().unwrap().parse::<usize>().unwrap() - 1;
        let end_city = input.next().unwrap().parse::<usize>().unwrap() - 1;
        let cost = input.next().unwrap().parse::<i32>().unwrap();

        citys[start_city].bus_index.as_mut().unwrap().push(Bus {
            cost,
            city_index: end_city,
        });
    }
    let start_city = input.next().unwrap().parse::<usize>().unwrap() - 1;
    let end_city = input.next().unwrap().parse::<usize>().unwrap() - 1;

    citys[start_city].cost = 0;

    let mut queue = BinaryHeap::new();
    queue.push(Reverse(CityIndex {
        cost: citys[start_city].cost,
        index: start_city,
    }));

    while let Some(Reverse(start_city)) = queue.pop() {
        if start_city.cost > citys[start_city.index].cost {
            continue;
        }

        let start_buses = citys[start_city.index].bus_index.take().unwrap();

        for &bus in start_buses.iter() {
            let cost = bus.cost + citys[start_city.index].cost;
            if cost < citys[bus.city_index].cost {
                citys[bus.city_index].cost = cost;
                queue.push(Reverse(CityIndex {
                    cost,
                    index: bus.city_index,
                }));
            }
        }

        citys[start_city.index].bus_index = Some(start_buses);
    }

    write!(writer, "{}", citys[end_city].cost).unwrap();
    writer.flush().unwrap();
}
