use std::{
    io::{stdout, Write},
    time::{SystemTime, UNIX_EPOCH},
};

// fn is_leap_year(year: u64) -> bool {
//     (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
// }
// fn days_in_month(year: u64, month: u64) -> u64 {
//     match month {
//         1 => 31,
//         2 => {
//             if is_leap_year(year) {
//                 29
//             } else {
//                 28
//             }
//         }
//         3 | 5 | 7 | 8 | 10 | 12 => 31,
//         4 | 6 | 9 | 11 => 30,
//         _ => 0,
//     }
// }
fn main() {
    // let now = SystemTime::now();
    // let duration_since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    // let total_seconds = duration_since_epoch.as_secs();
    // let minits = total_seconds / 60;
    // let hours = minits / 60 + 9;
    // let days = hours / 24;
    // let mut year = 1970;
    // let mut day = days;
    // while day >= 365 {
    //     if is_leap_year(year) {
    //         if day >= 366 {
    //             day -= 366;
    //             year += 1;
    //         } else {
    //             break;
    //         }
    //     } else {
    //         day -= 365;
    //         year += 1;
    //     }
    // }
    // let mut month = 1;
    // while day >= days_in_month(year, month) {
    //     day -= days_in_month(year, month);
    //     month += 1;
    // }
    // day += 1;
    stdout()
        // .write_all(format!("{year}-{month:02}-{day:02}").as_bytes())
        .write_all("2024-07-28".as_bytes())
        .unwrap();
}
