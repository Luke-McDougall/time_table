pub fn date_number(year: &str, month: &str, day: &str) -> i16 {
    let month: i16 = match month {
        "Jan" => 0,
        "Feb" => 31,
        "Mar" => 59,
        "Apr" => 90,
        "May" => 120,
        "Jun" => 151,
        "Jul" => 181,
        "Aug" => 212,
        "Sep" => 243,
        "Oct" => 273,
        "Nov" => 304,
        "Dec" => 334,
        _ => panic!("Something went wrong, no valid month found."),
    };

    let day: i16 = day.parse().unwrap();
    let year: i16 = year.parse().unwrap();
    if year % 4 == 0 && month > 31 {
        month + day + 1
    } else {
        month + day
    }
}
