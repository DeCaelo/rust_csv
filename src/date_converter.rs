use chrono::NaiveDate;

#[inline]
fn remove_prefix(date_str: &str) -> &str {
    date_str.trim_start_matches("Reviewed ")
}

pub fn convert_date(date_str: &str) -> Option<NaiveDate> {
    let date_str = remove_prefix(date_str);
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATE_STR: &str = "Reviewed Sept. 13, 2023";
    #[test]
    fn test_remove_prefix() {
        let date_str = remove_prefix(DATE_STR);
        assert_eq!(date_str, "Sept. 13, 2023")
    }

    #[test]
    fn test_convert_date() {
        let converted_date = convert_date(DATE_STR).unwrap();
        assert_eq!(
            converted_date,
            NaiveDate::from_ymd_opt(2023, 9, 13).expect("invalid date")
        );
    }
}
