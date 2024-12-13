use std::collections::HashMap;
use csv::ReaderBuilder;

// Processes the dataset by reading a CSV file and returning a vector of hash maps where each map represents a row.
pub fn process_dataset(file_path: &str) -> Vec<HashMap<String, String>> {
    let mut dataset = Vec::new();
    // Initializes a CSV reader with headers enabled.
    let reader = ReaderBuilder::new().has_headers(true).from_path(file_path);

    // Checks if the file is successfully read, and processes the records if it is.
    if let Ok(mut reader) = reader {
        let headers = reader.headers().unwrap().clone();

        for result in reader.records() {
            if let Ok(record) = result {
                let mut row = HashMap::new();
                for (key, value) in record.iter().enumerate() {
                    row.insert(headers[key].to_string(), value.to_string()); // Use the `headers` variable here
                }
                dataset.push(row);
            }
        }
    } else {
        eprintln!("Failed to read the file: {}", file_path);
    }

    dataset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataset_processing() {
        let processed_data = process_dataset("used_car_dataset.csv");
        assert!(!processed_data.is_empty(), "Processed dataset should not be empty.");
    }
}
