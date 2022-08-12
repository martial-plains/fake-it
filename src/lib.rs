use rand::Rng;

use crate::utils::get_random_country_phone_code;

pub mod datetime;
pub mod email;
pub mod person;
pub mod providers;
pub mod utils;

/// Generates a random phone number
pub fn generate_phone_number(cc: Option<String>) -> String {
    let mut rng = rand::thread_rng();

    let country_code = match cc {
        Some(cc) => cc,
        None => get_random_country_phone_code(),
    };
    let area_code: i32 = rng.gen_range(1..999);
    let first_three: i32 = rng.gen_range(1..999);
    let last_four: i32 = rng.gen_range(1..9999);

    format!(
        "+{}-{:03}-{:03}-{:04}",
        country_code, area_code, first_three, last_four
    )
}

/// Gets Country from a phone number
pub fn get_country_from_phone_number(phone_number: &str) -> Option<String> {
    let country_phone_codes = utils::get_country_phone_codes();

    for (country_name, country_code) in country_phone_codes.iter() {
        let cc = phone_number.split('-').next().unwrap();

        if cc.contains(country_code) {
            return Some(country_name.to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use log::info;

    use super::*;

    #[test]
    fn test_generate_phone_number_with_cc() {
        info!("Generating phone number with country code");
        let mut phone_number = generate_phone_number(Some("1".to_string()));
        info!("{}", phone_number);
        phone_number = phone_number.replace('-', "");
        assert!(phone_number.len() == 12);
    }
}
