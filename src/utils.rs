use std::{fs::File, io::Write};

use log::info;
use serde::Serialize;

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
