use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim().parse::<i32>().unwrap();

    let mut writer = BufWriter::new(stdout().lock());

    writer.write("int a;\n".as_bytes()).unwrap();
    writer.write("int *ptr = &a;\n".as_bytes()).unwrap();
    for i in 2..=input {
        writer.write("int ".as_bytes()).unwrap();
        for _ in 0..i {
            writer.write("*".as_bytes()).unwrap();
        }
        writer
            .write(
                format!(
                    "ptr{i} = &ptr{};\n",
                    if i - 1 == 1 {
                        "".to_owned()
                    } else {
                        format!("{}", i - 1)
                    }
                )
                .as_bytes(),
            )
            .unwrap();
    }
}
