use std::io::{stdout, Write};

fn main() {
    write!(
        stdout(),
        "         ,r'\"7
r`-_   ,'  ,/
 \\. \". L_r'
   `~\\/
      |
      |      "
    )
    .unwrap();
    stdout().flush().unwrap();
}
