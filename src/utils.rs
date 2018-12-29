use chrono::{NaiveDateTime, NaiveDate, Datelike};

pub fn pretty_print_date_range(
    from_option: &Option<NaiveDate>,
    to_option: &Option<NaiveDate>,
) -> String {
    if let (&Some(ref from), &Some(ref to)) = (from_option, to_option) {
        if from.month() == to.month() {
            format!(
                "{month} {day_from}-{day_to}",
                day_from = from.format("%d"),
                day_to = to.format("%d"),
                month = from.format("%b"),
            )
        } else if from.month() <= to.month() {
            format!(
                "{month_from} {day_from} - {month_to} {day_to}",
                day_from = from.format("%d"),
                month_from = from.format("%b"),
                day_to = to.format("%d"),
                month_to = to.format("%b"),
            )
        } else {
            let from_string = from.to_string();
            let to_string = to.to_string();
            format!("{} - {}", from_string, to_string)
        }
    } else {
        "TBD".to_string()
    }
}

pub fn pretty_print_session_date_and_time(timeoption: &Option<NaiveDateTime>) -> String {
    if let &Some(time) = timeoption {
        let day = time.format("%A").to_string();
        let hm = time.format("%H:%M").to_string();
        format!("{}, {}", day, hm)
    } else {
        format!("<i>TBD</i>")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_pretty_print_date_range() {
        {
            let from = Some(NaiveDate::from_ymd(2017, 3, 23));
            let to = Some(NaiveDate::from_ymd(2017, 3, 25));
            assert_eq!(
                pretty_print_date_range(&from, &to),
                String::from("Mar 23-25")
            );
        }
        {
            let from = Some(NaiveDate::from_ymd(2017, 4, 30));
            let to = Some(NaiveDate::from_ymd(2017, 5, 2));
            assert_eq!(
                pretty_print_date_range(&from, &to),
                String::from("Apr 30 - May 02")
            );
        }
    }
}
