use rand::Rng;

use crate::utils::get_random_country_phone_code;

pub mod datetime;
pub mod email;
pub mod person;
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

#[cfg(test)]
mod tests {
    use log::info;

    use super::*;

    #[test]
    fn test_generate_phone_number_with_cc() {
        info!("Generating phone number with country code");
        let mut phone_number = generate_phone_number(Some("1".to_string()));
        info!("{}", phone_number);
        phone_number = phone_number.replace("-", "");
        assert!(phone_number.len() == 12);
    }
}
