use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let mut count = input.next().unwrap().parse::<usize>().unwrap();
    let mut list = Vec::with_capacity(count);

    for i in 0..count {
        let temp = input.next().unwrap().parse::<i32>().unwrap();
        list.push(temp);
    }

    let mut answer = Vec::with_capacity(count * count);
    // 가장 큰 요소와 그 다음부터 가장 작은 요소를 뒤집는다?
    loop {
        if count == 0 {
            break;
        }
        let mut max = (0, 0);
        for i in 0..count {
            if max.0 < list[i] {
                max = (list[i], i);
            }
        }

        let mut min = (i32::MAX, 0);
        for i in max.1..count {
            if min.0 > list[i] {
                min = (list[i], i + 1);
            }
        }

        if max.1 == min.1 - 1 {
            count -= 1;
        } else {
            answer.push((max.1 + 1, min.1));
            list = reverse(list, max.1, min.1);
        }
    }

    let mut bw = BufWriter::new(stdout().lock());
    writeln!(bw, "{}", answer.len()).unwrap();
    for (a, b) in answer {
        writeln!(bw, "{a} {b}").unwrap();
    }
    bw.flush().unwrap();
}

fn reverse(mut list: Vec<i32>, x: usize, y: usize) -> Vec<i32> {
    let lsplit = &mut list[x..y];
    lsplit.reverse();
    // println!("{:?}", list);

    list
}
