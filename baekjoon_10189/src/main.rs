use std::io::{stdout, Write};

fn main() {
    let tab_double = "#  #";
    let just_double = "# #";
    let just_four = "####";
    let updown = format!("{tab_double} {just_four} {just_four} {tab_double} ");
    let middle = format!("{just_four} {tab_double} {tab_double} {just_double} ");
    stdout()
        .write_all(format!("{updown}\n{middle}\n{middle}\n{updown}").as_bytes())
        .unwrap();
}
