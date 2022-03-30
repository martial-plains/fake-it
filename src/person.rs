use chrono::{DateTime, Datelike, TimeZone, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::datetime;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Person {
    first_name: String,
    surname: String,
    date_of_birth: DateTime<Utc>,
}

impl Person {
    /// Get a reference to the person's first name.
    #[must_use]
    pub fn first_name(&self) -> &str {
        self.first_name.as_ref()
    }

    /// Get a reference to the person's surname.
    #[must_use]
    pub fn surname(&self) -> &str {
        self.surname.as_ref()
    }

    /// Get the person's date of birth.
    #[must_use]
    pub fn date_of_birth(&self) -> DateTime<Utc> {
        self.date_of_birth
    }
}

impl Default for Person {
    fn default() -> Person {
        let mut rng = rand::thread_rng();
        Person {
            first_name: String::from("John"),
            surname: String::from("Doe"),
            date_of_birth: {
                let d = Utc::now().date();
                let current_year = d.year();
                let current_month = d.month();
                let current_day = d.day();

                let year = rng.gen_range(1900..current_year);
                if year == current_year {
                    let month = rng.gen_range(1..current_month);
                    if month == current_month {
                        let day = rng.gen_range(1..current_day);
                        Utc.ymd(year, month, day).and_hms(0, 0, 0)
                    } else {
                        datetime::get_random_date_in_month(year, month)
                    }
                } else {
                    let month: u32 = rng.gen_range(1..12);
                    datetime::get_random_date_in_month(year, month)
                }
            },
        }
    }
}
