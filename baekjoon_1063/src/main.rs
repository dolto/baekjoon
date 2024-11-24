use std::io::{stdin, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let mut king = input.next().unwrap().chars().collect::<Vec<char>>();
    let mut stone = input.next().unwrap().chars().collect::<Vec<char>>();

    (0..input.next().unwrap().parse().unwrap()).for_each(|_| match input.next().unwrap() {
        "B" => {
            king[1] = if king[1] != '1' {
                (king[1] as u8 - 1) as char
            } else {
                king[1]
            };
            if king == stone {
                stone[1] = if stone[1] != '1' {
                    (stone[1] as u8 - 1) as char
                } else {
                    king[1] = '2';
                    stone[1]
                };
            }
        }
        "T" => {
            king[1] = if king[1] != '8' {
                (king[1] as u8 + 1) as char
            } else {
                king[1]
            };
            if king == stone {
                stone[1] = if stone[1] != '8' {
                    (stone[1] as u8 + 1) as char
                } else {
                    king[1] = '7';
                    stone[1]
                };
            }
        }
        "R" => {
            king[0] = if king[0] != 'H' {
                (king[0] as u8 + 1) as char
            } else {
                king[0]
            };
            if king == stone {
                stone[0] = if stone[0] != 'H' {
                    (stone[0] as u8 + 1) as char
                } else {
                    king[0] = 'G';
                    stone[0]
                };
            }
        }
        "L" => {
            king[0] = if king[0] != 'A' {
                (king[0] as u8 - 1) as char
            } else {
                king[0]
            };
            if king == stone {
                stone[0] = if stone[0] != 'A' {
                    (stone[0] as u8 - 1) as char
                } else {
                    king[0] = 'B';
                    stone[0]
                };
            }
        }
        "RT" => {
            if king[1] != '8' && king[0] != 'H' {
                king[1] = (king[1] as u8 + 1) as char;
                king[0] = (king[0] as u8 + 1) as char;
                if king == stone {
                    if stone[1] != '8' && stone[0] != 'H' {
                        stone[1] = (stone[1] as u8 + 1) as char;
                        stone[0] = (stone[0] as u8 + 1) as char;
                    } else {
                        king[1] = (king[1] as u8 - 1) as char;
                        king[0] = (king[0] as u8 - 1) as char;
                    }
                }
            }
        }
        "LT" => {
            if king[1] != '8' && king[0] != 'A' {
                king[1] = (king[1] as u8 + 1) as char;
                king[0] = (king[0] as u8 - 1) as char;
                if king == stone {
                    if stone[1] != '8' && stone[0] != 'A' {
                        stone[1] = (stone[1] as u8 + 1) as char;
                        stone[0] = (stone[0] as u8 - 1) as char;
                    } else {
                        king[1] = (king[1] as u8 - 1) as char;
                        king[0] = (king[0] as u8 + 1) as char;
                    }
                }
            }
        }
        "RB" => {
            if king[1] != '1' && king[0] != 'H' {
                king[1] = (king[1] as u8 - 1) as char;
                king[0] = (king[0] as u8 + 1) as char;
                if king == stone {
                    if stone[1] != '1' && stone[0] != 'H' {
                        stone[1] = (stone[1] as u8 - 1) as char;
                        stone[0] = (stone[0] as u8 + 1) as char;
                    } else {
                        king[1] = (king[1] as u8 + 1) as char;
                        king[0] = (king[0] as u8 - 1) as char;
                    }
                }
            }
        }
        "LB" => {
            if king[1] != '1' && king[0] != 'A' {
                king[1] = (king[1] as u8 - 1) as char;
                king[0] = (king[0] as u8 - 1) as char;
                if king == stone {
                    if stone[1] != '1' && stone[0] != 'A' {
                        stone[1] = (stone[1] as u8 - 1) as char;
                        stone[0] = (stone[0] as u8 - 1) as char;
                    } else {
                        king[1] = (king[1] as u8 + 1) as char;
                        king[0] = (king[0] as u8 + 1) as char;
                    }
                }
            }
        }
        _ => {}
    });

    let mut bw = BufWriter::new(stdout().lock());
    write!(bw, "{}{}\n{}{}", king[0], king[1], stone[0], stone[1]).unwrap();
    bw.flush().unwrap();
}
