use chrono::NaiveDate;
use rust_csv::date_converter::deserialize_date;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Review {
    name: String,
    location: String,
    #[serde(rename = "Date", deserialize_with = "deserialize_date")]
    date: NaiveDate,
    #[serde(rename = "Rating")]
    rating: String,
    #[serde(rename = "Review")]
    review: String,
    #[serde(rename = "Image_Links")]
    image_links: String,
}

fn main() {
    let mut csv_reader = csv::Reader::from_path("reviews_data.csv").unwrap();
    let mut review_rows = csv_reader
        .deserialize()
        .map(|r| r.unwrap())
        .collect::<Vec<Review>>();
    println!("Read {} rows", review_rows.len());
    review_rows.sort_by_key(|r| r.date);
    review_rows.iter().take(3).for_each(|r| println!("{:?}", r));
}
