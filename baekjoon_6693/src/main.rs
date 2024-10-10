use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split(".");

    let mut writer = BufWriter::new(stdout().lock());
    while let Some(input) = input.next() {
        // println!("{input}");
        if input.trim() == "END" {
            break;
        } else {
            let (mut x, mut y) = (0., 0.);
            let input = input.split(",");
            for go in input {
                let go = go.trim().chars();
                let mut value = String::new();
                let mut direction = String::new();
                for g in go {
                    match g {
                        '0' => value.push_str(g.to_string().as_str()),
                        '1' => value.push_str(g.to_string().as_str()),
                        '2' => value.push_str(g.to_string().as_str()),
                        '3' => value.push_str(g.to_string().as_str()),
                        '4' => value.push_str(g.to_string().as_str()),
                        '5' => value.push_str(g.to_string().as_str()),
                        '6' => value.push_str(g.to_string().as_str()),
                        '7' => value.push_str(g.to_string().as_str()),
                        '8' => value.push_str(g.to_string().as_str()),
                        '9' => value.push_str(g.to_string().as_str()),
                        _ => direction.push_str(g.to_string().as_str()),
                    }
                }

                let value = value.parse::<f64>().unwrap();
                match direction.as_str() {
                    "N" => y += value,
                    "S" => y -= value,
                    "E" => x += value,
                    "W" => x -= value,
                    "NE" => {
                        y += value / 2_f64.sqrt();
                        x += value / 2_f64.sqrt();
                    }
                    "NW" => {
                        y += value / 2_f64.sqrt();
                        x -= value / 2_f64.sqrt();
                    }
                    "SE" => {
                        y -= value / 2_f64.sqrt();
                        x += value / 2_f64.sqrt();
                    }
                    "SW" => {
                        y -= value / 2_f64.sqrt();
                        x -= value / 2_f64.sqrt();
                    }
                    _ => {}
                }
            }
            let distance = (x.powi(2) + y.powi(2)).sqrt();
            writer
                .write(
                    format!(
                        "You can go to ({:.3},{:.3}), the distance is {:.3} steps.\n",
                        x, y, distance
                    )
                    .as_bytes(),
                )
                .unwrap();
        }
    }

    writer.flush().unwrap();
}
