use std::{fs::File, io::Write};

use log::info;
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
