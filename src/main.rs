use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Review {
    name: String,
    location: String,
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Rating")]
    rating: String,
    #[serde(rename = "Review")]
    review: String,
    #[serde(rename = "Image_Links")]
    image_links: String,
}

fn main() {
    let mut csv_reader = csv::Reader::from_path("reviews_data.csv").unwrap();
    let review_rows = csv_reader
        .deserialize()
        .map(|r| r.unwrap())
        .collect::<Vec<Review>>();
    println!("Read {} rows", review_rows.len());
}
