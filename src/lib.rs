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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_from_csv_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.csv");

        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Index,Customer Id,First Name,Last Name,Company,City,Country,Phone 1,Phone 2,Email,Subscription Date,Website").unwrap();
        writeln!(file, "1,C123,John,Doe,Acme Inc.,New York,USA,1234567890,0987654321,john.doe@example.com,2021-01-01,www.example.com").unwrap();

        let records = CsvRecord::from_csv_file(&file_path).unwrap();
        assert_eq!(records.len(), 1);

        let record = &records[0];
        assert_eq!(record.index, 1);
        assert_eq!(record.customer_id, "C123");
        assert_eq!(record.first_name, "John");
        assert_eq!(record.last_name, "Doe");
        assert_eq!(record.company, "Acme Inc.");
        assert_eq!(record.city, "New York");
        assert_eq!(record.country, "USA");
        assert_eq!(record.phone1, "1234567890");
        assert_eq!(record.phone2, "0987654321");
        assert_eq!(record.email, "john.doe@example.com");
        assert_eq!(record.subscription_date, "2021-01-01");
        assert_eq!(record.website, "www.example.com");
    }

    #[test]
    fn test_from_csv_file_empty() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("empty.csv");

        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Index,Customer Id,First Name,Last Name,Company,City,Country,Phone 1,Phone 2,Email,Subscription Date,Website").unwrap();

        let records = CsvRecord::from_csv_file(&file_path).unwrap();
        assert!(records.is_empty());
    }

    #[test]
    fn test_from_csv_file_invalid() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("invalid.csv");

        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Index,Customer Id,First Name,Last Name,Company,City,Country,Phone 1,Phone 2,Email,Subscription Date,Website").unwrap();
        writeln!(file, "invalid,data,that,does,not,match,the,structure").unwrap();

        let result = CsvRecord::from_csv_file(&file_path);
        assert!(result.is_err());
    }
}
