use std::{
    collections::HashSet,
    io::{stdin, stdout, Read, Write},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let player: usize = input.next().unwrap().parse().unwrap();
    let goal: usize = input.next().unwrap().parse().unwrap();

    if goal < player {
        stdout()
            .write_all(format!("{}", player - goal).as_bytes())
            .unwrap();
        return;
    } else if goal == player {
        stdout().write_all("0".as_bytes()).unwrap();
        return;
    }

    let mut sec = 0;
    let mut such_vec = vec![false; (player.max(goal) + 1) * 2];
    let mut such_set = HashSet::new();
    such_set.insert(player + 1);
    such_set.insert(player - 1);
    such_set.insert(player * 2);
    such_vec[player] = true;

    'such: loop {
        sec += 1;
        let mut temp_set = HashSet::with_capacity(such_set.len() * 3);
        for &such in such_set.iter() {
            if such >= such_vec.len() || such_vec[such] {
                continue;
            }
            if such == goal {
                break 'such;
            } else {
                such_vec[such] = true;
                temp_set.insert(such + 1);
                if such != 0 {
                    temp_set.insert(such - 1);
                }
                temp_set.insert(such * 2);
            }
        }
        such_set = temp_set;
    }

    stdout().write_all(format!("{sec}").as_bytes()).unwrap();
}

// fn find_goal(goal: i32, log: &Vec<i32>) -> i32 {
//     if goal == 1 {
//         1
//     } else if goal == 2 {
//         2
//     } else {
//         if goal % 2 == 0 {
//             log[goal as usize / 2].min(log[goal as usize - 1]) + 1
//         } else {
//             log[goal as usize - 1] + 1
//         }
//     }
// }
