use chrono::{DateTime, UTC, Datelike, Local};

pub fn pretty_print_date_range(from: &DateTime<UTC>, to: &DateTime<UTC>) -> String {
    if from.month() == to.month() {
        format!("{day_from}-{day_to}, {month}",
                day_from = from.format("%d"),
                day_to = to.format("%d"),
                month = from.format("%B"),
        )
    } else if from.month() <= to.month() {
        format!("{day_from} {month_from} - {day_to} {month_to}",
                day_from = from.format("%d"),
                month_from = from.format("%B"),
                day_to = to.format("%d"),
                month_to = to.format("%B"),
        )
    } else {
        let from_string = from.to_string();
        let to_string = to.to_string();
        format!("{} - {}", from_string, to_string)
    }
}

pub fn pretty_print_session_date_and_time(date: &DateTime<UTC>, timeoption: &Option<DateTime<Local>>) -> String {
    let day = date.format("%A").to_string();
    if let &Some(time) = timeoption {
        let hm = time.format("%H%M").to_string();
        format!("{}, {}", day, hm)
    } else {
        format!("{}, <i>TBD</i>", day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{UTC, TimeZone};

    #[test]
    fn test_pretty_print_date_range() {
        {
            let from = UTC.ymd(2017, 3, 23).and_hms(0, 0, 0);
            let to = UTC.ymd(2017, 3, 25).and_hms(0, 0, 0);
            assert_eq!(pretty_print_date_range(&from, &to), String::from("23-25, March"));
        }
        {
            let from = UTC.ymd(2017, 4, 30).and_hms(0, 0, 0);
            let to = UTC.ymd(2017, 5, 2).and_hms(0, 0, 0);
            assert_eq!(pretty_print_date_range(&from, &to), String::from("30 April - 02 May"));
        }
    }
}
