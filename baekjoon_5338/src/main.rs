use std::io::{stdout, BufWriter, StdoutLock, Write};

fn main() {
    let mut writer = BufWriter::new(stdout().lock());
    writer.write("       ".as_bytes()).unwrap();
    haed(&mut writer);
    tail(&mut writer);
    haed2(&mut writer);
    tail(&mut writer);
    haed3(&mut writer);
    tail(&mut writer);
    haed2(&mut writer);
    tail(&mut writer);
    haed4(&mut writer);
    writer.flush().unwrap();
}

fn tail(writer: &mut BufWriter<StdoutLock>) {
    writer.write("'-..-'".as_bytes()).unwrap();
}

fn haed(writer: &mut BufWriter<StdoutLock>) {
    writer.write("_.-;;-._\n".as_bytes()).unwrap();
}
fn haed2(writer: &mut BufWriter<StdoutLock>) {
    writer.write("|   ||   |\n".as_bytes()).unwrap();
}
fn haed3(writer: &mut BufWriter<StdoutLock>) {
    writer.write("|_.-;;-._|\n".as_bytes()).unwrap();
}
fn haed4(writer: &mut BufWriter<StdoutLock>) {
    writer.write("|_.-''-._|\n".as_bytes()).unwrap();
}
