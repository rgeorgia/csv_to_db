use serde::Deserialize;
use std::error::Error;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct CsvRecord {
    #[serde(rename = "Index")]
    pub index: u32,
    #[serde(rename = "Customer Id")]
    pub customer_id: String,
    #[serde(rename = "First Name")]
    pub first_name: String,
    #[serde(rename = "Last Name")]
    pub last_name: String,
    #[serde(rename = "Company")]
    pub company: String,
    #[serde(rename = "City")]
    pub city: String,
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "Phone 1")]
    pub phone1: String,
    #[serde(rename = "Phone 2")]
    pub phone2: String,
    #[serde(rename = "Email")]
    pub email: String,
    #[serde(rename = "Subscription Date")]
    pub subscription_date: String,
    #[serde(rename = "Website")]
    pub website: String,
}

impl CsvRecord {
    pub fn from_csv_file(file_path: &Path) -> Result<Vec<CsvRecord>, Box<dyn Error>> {
        let mut records = Vec::new();
        let mut rdr = csv::Reader::from_path(file_path)?;
        for result in rdr.deserialize() {
            let record: CsvRecord = result?;
            records.push(record);
        }
        Ok(records)
    }
}
