use rand::Rng;
use regex::Regex;

use crate::person::Person;

/// Regular expression to match a valid email address
#[allow(dead_code)]
const REGEX: &str = r"(?x)^(?P<login>[^@\s]+)@([[:word:]]+\.)*[[:word:]]+$";

/// Validates email address
///
/// # Arguments
///
/// * `email` - email address to validate
#[allow(dead_code)]
fn is_valid(email: &str) -> bool {
    let re = Regex::new(REGEX).unwrap();
    re.is_match(email)
}

/// Generate a random email address
///
/// # Arguments
///
/// * `person` - person to generate email address for
pub fn generate(person: Option<Person>) -> String {
    let mut rng = rand::thread_rng();
    let person = person.unwrap_or_default();

    let domains = vec![
        "gmail", "yahoo", "outlook", "express", "yandex", "nexus", "icloud",
    ];
    let extensions = vec![
        "com", "in", "jp", "us", "uk", "org", "edu", "au", "de", "co", "me", "biz", "dev", "ngo",
        "site", "xyz", "zero", "tech",
    ];

    let c: u8 = rng.gen_range(0..2);
    let domain = format!("@{}", domains[rng.gen_range(0..domains.len())]);
    let extension = format!(".{}", extensions[rng.gen_range(0..extensions.len())]);

    match c {
        0 => {
            format!(
                "{}{}{}{}",
                person.first_name(),
                person.date_of_birth().format("%Y"),
                domain,
                extension
            )
        }
        1 => {
            format!(
                "{}{}{}{}",
                person.surname(),
                person.date_of_birth().format("%d"),
                domain,
                extension
            )
        }
        _ => {
            format!(
                "{}{}{}{}",
                person.first_name(),
                person.date_of_birth().format("%y"),
                domain,
                extension
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let email = generate(None);
        println!("{}", email);
        assert!(is_valid(&email));
    }
}
