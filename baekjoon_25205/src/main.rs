use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .chars()
        .rev();

    let mut ch = input.next().unwrap();
    let mut ok = match ch {
        't' => {
            ch = input.next().unwrap();
            match ch {
                't' | 'q' | 'f' | 'r' => {
                    if let Some(c) = input.next() {
                        ch = c;
                        true
                    } else {
                        false
                    }
                }
                _ => true,
            }
        }
        'g' => {
            ch = input.next().unwrap();
            match ch {
                'f' | 's' => {
                    if let Some(c) = input.next() {
                        ch = c;
                        true
                    } else {
                        false
                    }
                }
                _ => true,
            }
        }
        'v' | 'x' | 'q' | 'a' => {
            ch = input.next().unwrap();
            match ch {
                'f' => {
                    if let Some(c) = input.next() {
                        ch = c;
                        true
                    } else {
                        false
                    }
                }
                _ => true,
            }
        }
        'w' => {
            ch = input.next().unwrap();
            match ch {
                's' => {
                    if let Some(c) = input.next() {
                        ch = c;
                        true
                    } else {
                        false
                    }
                }
                _ => true,
            }
        }
        'r' => {
            ch = input.next().unwrap();
            match ch {
                'r' | 'f' => {
                    if let Some(c) = input.next() {
                        ch = c;
                        true
                    } else {
                        false
                    }
                }
                _ => true,
            }
        }
        's' | 'e' | 'f' | 'd' | 'c' | 'z' => {
            ch = input.next().unwrap();
            true
        }
        _ => false,
    };

    if ok {
        ok = match ch {
            'l' => {
                if let Some(c) = input.next() {
                    ch = c;
                    match ch {
                        'm' | 'n' | 'h' => {
                            if let Some(c) = input.next() {
                                ch = c;
                                true
                            } else {
                                false
                            }
                        }
                        _ => true,
                    }
                } else {
                    false
                }
            }
            'p' | 'o' => {
                if let Some(c) = input.next() {
                    ch = c;
                    match ch {
                        'n' | 'h' => {
                            if let Some(c) = input.next() {
                                ch = c;
                                true
                            } else {
                                false
                            }
                        }
                        _ => true,
                    }
                } else {
                    false
                }
            }
            'j' => {
                if let Some(c) = input.next() {
                    ch = c;
                    match ch {
                        'n' => {
                            if let Some(c) = input.next() {
                                ch = c;
                                true
                            } else {
                                false
                            }
                        }
                        _ => true,
                    }
                } else {
                    false
                }
            }
            'k' => {
                if let Some(c) = input.next() {
                    ch = c;
                    match ch {
                        'h' => {
                            if let Some(c) = input.next() {
                                ch = c;
                                true
                            } else {
                                false
                            }
                        }
                        _ => true,
                    }
                } else {
                    false
                }
            }
            // ㅑㅕㅗㅛㅜㅠㅡㅒㅖ ㅣㅐㅔㅓㅏ
            'i' | 'u' | 'h' | 'n' | 'b' | 'm' | 'O' | 'P' | 'y' => {
                if let Some(c) = input.next() {
                    ch = c;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    if ok {
        ok = match ch {
            'r' | 's' | 'e' | 'f' | 'a' | 'q' | 't' | 'd' | 'w' | 'c' | 'z' | 'x' | 'v' | 'g' => {
                true
            }
            _ => false,
        }
    }
    if ok {
        stdout().write_all("1".as_bytes()).unwrap();
    } else {
        stdout().write_all("0".as_bytes()).unwrap();
    }
}
