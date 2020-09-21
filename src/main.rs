use chrono::{Utc, TimeZone, DateTime};

fn main() {
    let now: DateTime<Utc> = Utc::now();
    println!("{}", now); // 2020-09-21 06:35:15.767329 UTC

    let some_date: DateTime<Utc> =  Utc.ymd(2020, 1, 21).and_hms(22, 2, 3);
    println!("{}", some_date); // 2020-01-21 22:02:03 UTC
}
