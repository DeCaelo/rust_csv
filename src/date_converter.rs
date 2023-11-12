use chrono::NaiveDate;

#[inline]
fn remove_prefix(date_str: &str) -> &str {
    date_str.trim_start_matches("Reviewed ")
}

#[repr(u32)]
enum Months {
    Jan = 1,
    Feb = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    Aug = 8,
    Sept = 9,
    Oct = 10,
    Nov = 11,
    Dec = 12,
}

fn date_split(date_str: &str) -> Option<(&str, &str, &str)> {
    let mut parts = date_str.split_whitespace();
    let month = parts.next()?;
    let day = parts.next()?;
    let year = parts.next()?;
    Some((month, day, year))
}

impl std::str::FromStr for Months {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Jan." => Ok(Months::Jan),
            "Feb." => Ok(Months::Feb),
            "March" => Ok(Months::March),
            "April" => Ok(Months::April),
            "May" => Ok(Months::May),
            "June" => Ok(Months::June),
            "July" => Ok(Months::July),
            "Aug." => Ok(Months::Aug),
            "Sept." => Ok(Months::Sept),
            "Oct." => Ok(Months::Oct),
            "Nov." => Ok(Months::Nov),
            "Dec." => Ok(Months::Dec),
            _ => Err("invalid month"),
        }
    }
}

impl From<Months> for u32 {
    fn from(month: Months) -> Self {
        month as u32
    }
}

pub fn convert_date(date_str: &str) -> Option<NaiveDate> {
    let date_str = remove_prefix(date_str);
    let (month_str, day_str, year_str) = date_split(date_str)?;
    let month = month_str.parse::<Months>().ok()?.into();
    let day = day_str.trim_end_matches(',').parse::<u32>().ok()?;
    let year = year_str.parse::<i32>().ok()?;
    NaiveDate::from_ymd_opt(year, month, day)
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
