use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    let mut reader = BufReader::new(stdin().lock());
    reader.read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let mut writer = BufWriter::new(stdout().lock());
    let antenna: i32 = input.next().unwrap().trim().parse().unwrap();
    let eyes: i32 = input.next().unwrap().trim().parse().unwrap();
    // let mut candidate = Vec::with_capacity(3);

    let ailian = ["TroyMartian", "VladSaturnian", "GraemeMercurian"];

    if 3 <= antenna && 4 >= eyes {
        writer
            .write_all(format!("{}\n", ailian[0]).as_bytes())
            .unwrap();
    }
    if 6 >= antenna && 2 <= eyes {
        writer
            .write_all(format!("{}\n", ailian[1]).as_bytes())
            .unwrap();
    }
    if 2 >= antenna && 3 >= eyes {
        writer
            .write_all(format!("{}\n", ailian[2]).as_bytes())
            .unwrap();
    }
}
