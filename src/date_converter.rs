use chrono::NaiveDate;
use serde::de::Deserialize;

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

fn date_split(date_str: &str) -> Result<(&str, &str, &str), &'static str> {
    let mut parts = date_str.split_whitespace();
    let month = parts.next().ok_or("No month")?;
    let day = parts.next().ok_or("No day")?;
    let year = parts.next().ok_or("No year")?;
    Ok((month, day, year))
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

#[inline]
fn parse_day_str(day_str: &str) -> Result<u32, &'static str> {
    day_str
        .trim_end_matches(',')
        .parse::<u32>()
        .map_err(|_| "Invalid day")
}

#[inline]
fn parse_year_str(year_str: &str) -> Result<i32, &'static str> {
    year_str.parse::<i32>().map_err(|_| "Invalid year")
}

pub fn convert_date(date_str: &str) -> Result<NaiveDate, &'static str> {
    let date_str = remove_prefix(date_str);
    let (month_str, day_str, year_str) = date_split(date_str)?;
    let month = month_str.parse::<Months>()?.into();
    let day = parse_day_str(day_str)?;
    let year = parse_year_str(year_str)?;
    NaiveDate::from_ymd_opt(year, month, day).ok_or("Invalid date")
}

pub fn deserialize_date<'de, D>(deserialiser: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_str: &str = Deserialize::deserialize(deserialiser)?;
    convert_date(date_str).map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATE_STR: &str = "Reviewed Sept. 13, 2023";
    const DATE_STR_2: &str = "Reviewed June 30, 2020";
    const DATE_STR_3: &str = "Reviewed Jan. 1, 2001";

    #[test]
    fn test_remove_prefix() {
        let date_str = remove_prefix(DATE_STR);
        assert_eq!(date_str, "Sept. 13, 2023")
    }

    #[test]
    fn test_convert_date() {
        assert_eq!(
            convert_date(DATE_STR).unwrap(),
            NaiveDate::from_ymd_opt(2023, 9, 13).expect("invalid date")
        );

        assert_eq!(
            convert_date(DATE_STR_2).unwrap(),
            NaiveDate::from_ymd_opt(2020, 6, 30).expect("invalid date")
        );

        assert_eq!(
            convert_date(DATE_STR_3).unwrap(),
            NaiveDate::from_ymd_opt(2001, 1, 1).expect("invalid date")
        );
    }
}
