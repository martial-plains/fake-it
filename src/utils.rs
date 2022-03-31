use std::{collections::HashMap, fs::File, io::Write};

use log::info;
use rand::Rng;
use serde::Serialize;

/// Export data to a JSON file
///
/// # Arguments
///
/// * `data` - data to export
/// * `path` - path to export to
pub fn export_to_json<T>(data: &T, path: &str) -> Result<(), serde_json::Error>
where
    T: Serialize,
{
    let json = serde_json::to_string(&data).unwrap();
    let mut file = File::create(path).unwrap();

    info!("Writing to file: {}", path);
    file.write_all(json.as_bytes()).unwrap();

    Ok(())
}

/// Export data to a CSV file
///
/// # Arguments
///
/// * `data` - data to export
/// * `path` - path to export to
pub fn export_to_csv<T>(data: &Vec<T>, path: &str) -> Result<(), std::io::Error>
where
    T: Serialize,
{
    let mut csv = csv::Writer::from_path(path).unwrap();

    info!("Writing to file: {}", path);

    for item in data {
        csv.serialize(item).unwrap();
    }

    Ok(())
}

/// Export data to an HTML
///
/// # Arguments
///
/// * `data` - data to export
/// * `path` - path to export to
pub fn export_to_html<T>(data: &Vec<T>, path: &str) -> Result<(), std::io::Error>
where
    T: Serialize,
{
    let mut html = File::create(path).unwrap();

    info!("Writing to file: {}", path);

    html.write_all(
        format!(
            "<!DOCTYPE html><html><head><title>{}</title></head><body><table><tr>",
            path
        )
        .as_bytes(),
    )
    .unwrap();

    for item in data {
        html.write_all(
            format!("<tr><td>{}</td></tr>", serde_json::to_string(item).unwrap()).as_bytes(),
        )
        .unwrap();
    }

    html.write_all("</table></body></html>".as_bytes()).unwrap();

    Ok(())
}

/// Gets all country codes from a CSV file
pub fn get_country_phone_codes() -> HashMap<String, String> {
    let mut file = csv::Reader::from_path("assets/data/country_phone_code.csv").unwrap();
    let mut country_phone_codes: HashMap<String, String> = HashMap::new();

    file.records().for_each(|record| {
        let record = record.unwrap();
        let name = record[0].to_string();
        let code = record[1].to_string();
        country_phone_codes.insert(name, code);
    });

    country_phone_codes
}

/// Gets a random country code from a CSV file
pub fn get_random_country_phone_code() -> String {
    let mut rng = rand::thread_rng();
    let country_phone_codes = get_country_phone_codes();
    let country_codes: Vec<String> = country_phone_codes
        .keys()
        .map(|key| key.to_string())
        .collect();
    let random_country_code = &country_codes[rng.gen_range(0..country_codes.len())];

    random_country_code.to_string()
}

/// Gets a country code from a CSV file by name
pub fn get_country_phone_code_by_name(name: &str) -> String {
    let country_phone_codes = get_country_phone_codes();
    let country_code = country_phone_codes.get(name).unwrap();

    country_code.to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_get_random_country_phone_code() {
        let country_phone_codes = get_country_phone_codes();
        let country_codes: Vec<String> = country_phone_codes
            .keys()
            .map(|key| key.to_string())
            .collect();
        let random_country_code = get_random_country_phone_code();
        assert!(country_codes.contains(&random_country_code));
    }

    #[test]
    fn test_get_usa_phone_code() {
        let country_code = get_country_phone_code_by_name("United States");
        assert_eq!(country_code, "1");
    }

    #[test]
    fn test_export_to_json() {
        let data = "Hello World";
        let path = "test.json";
        export_to_json(&data, &path).unwrap();
        assert!(std::path::Path::new(&path).exists());
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_export_to_csv() {
        let data = vec!["Hello World"];
        let path = "test.csv";
        export_to_csv(&data, &path).unwrap();
        assert!(std::path::Path::new(&path).exists());
        fs::remove_file(&path).unwrap();
    }
}
