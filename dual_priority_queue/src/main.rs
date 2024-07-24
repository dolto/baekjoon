// 아이디어
// 구조체를 만든다 (&Option<i32>)
// 구조체의 Ord를 구현한다.
// 해당 구조체에 관한 최대힙을 만들고, i32에대한 최소힙을 만든다.
// 소유권은 최소힙이 가져가고, 최대힙의 요소들은 언제나 None이 될 수 있다는걸 명시하고
// 최대힙에서 Top을 꺼낼 때, 값이 None인경우, 제거하고, None이 아닐 때 까지 꺼낸다면?

// struct Ele(pub Weak<i32>);
// impl PartialEq for Ele {
//     fn eq(&self, other: &Self) -> bool {
//         let se = self.0.upgrade();
//         let oe = other.0.upgrade();
//         se == oe
//     }
// }
// impl Eq for Ele {}
// impl PartialOrd for Ele {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         let se = self.0.upgrade();
//         let oe = other.0.upgrade();
//         if let (Some(se_temp), Some(oe_temp)) = (se.clone(), oe.clone()) {
//             if se_temp > oe_temp {
//                 Some(Ordering::Greater)
//             } else if se_temp < oe_temp {
//                 Some(Ordering::Less)
//             } else {
//                 Some(Ordering::Equal)
//             }
//         } else if let Some(_) = se {
//             Some(Ordering::Greater)
//         } else if let Some(_) = oe {
//             Some(Ordering::Less)
//         } else {
//             Some(Ordering::Equal)
//         }
//     }
// }
// impl Ord for Ele {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         let se = self.0.upgrade();
//         let oe = other.0.upgrade();
//         if let (Some(se_temp), Some(oe_temp)) = (se.clone(), oe.clone()) {
//             if se_temp > oe_temp {
//                 Ordering::Greater
//             } else if se_temp < oe_temp {
//                 Ordering::Less
//             } else {
//                 Ordering::Equal
//             }
//         } else if let Some(_) = se {
//             Ordering::Greater
//         } else if let Some(_) = oe {
//             Ordering::Less
//         } else {
//             Ordering::Equal
//         }
//     }
// }
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{stdin, stdout, BufReader, Read, Write},
    rc::Rc,
};
fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    let mut output = String::new();

    reader.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let heap_count: usize = input.next().unwrap().trim().parse().unwrap();

    for _ in 0..heap_count {
        let try_count: usize = input.next().unwrap().trim().parse().unwrap();
        let mut min_heap = BinaryHeap::with_capacity(try_count);
        let mut max_heap = BinaryHeap::with_capacity(try_count);
        for _ in 0..try_count {
            let command = input.next().unwrap().trim();
            let res: i32 = input.next().unwrap().trim().parse().unwrap();

            if command == "I" {
                let res = Rc::new(res);
                min_heap.push(Reverse(Rc::clone(&res)));
                max_heap.push(Rc::clone(&res));
            } else if res == 1 {
                while let Some(max) = max_heap.pop() {
                    if Rc::strong_count(&max) >= 2 {
                        // println!("max pop: {}", max);
                        break;
                    }
                }
            } else if res == -1 {
                while let Some(Reverse(min)) = min_heap.pop() {
                    if Rc::strong_count(&min) >= 2 {
                        // println!("max pop: {}", min);
                        break;
                    }
                }
            }
        }
        while let Some(max) = max_heap.pop() {
            if Rc::strong_count(&max) >= 2 {
                max_heap.push(max);
                break;
            }
        }
        while let Some(Reverse(min)) = min_heap.pop() {
            if Rc::strong_count(&min) >= 2 {
                min_heap.push(Reverse(min));
                break;
            }
        }

        if let Some(max) = max_heap.peek() {
            let min = *min_heap.peek().unwrap().0;
            output += format!("{} {}\n", max, min).as_str();
        } else {
            output += "EMPTY\n";
        }
    }

    stdout().write_all(output.as_bytes()).unwrap();
}
