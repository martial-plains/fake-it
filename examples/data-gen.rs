use fake_it::{generate_phone_number, get_country_from_phone_number, utils::export_to_json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum SaleStatus {
    Pending,
    Completed,
    Returned,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum PaymentStatus {
    Pending,
    Due,
    Paid,
    Refunded,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum Gender {
    NotSpecified,
    Male,
    Female,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
enum TaxMethod {
    Exclusive,
    Inclusive,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Address {
    pub id: usize,

    pub line1: String,

    pub line2: String,

    pub city: String,

    pub country: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Person {
    id: Uuid,

    name: String,

    email: String,

    phone_number: String,

    gender: Gender,

    address: Address,
}

fn main() {
    let persons: Vec<Person> = (0..50)
        .map(|_| {
            let person = fake_it::person::Person::default();
            let phone_number = generate_phone_number(None);
            Person {
                id: Uuid::new_v4(),
                name: person.first_name().to_string(),
                email: fake_it::email::generate(Some(person)),
                phone_number: phone_number.clone(),
                gender: Gender::NotSpecified,
                address: Address {
                    id: 0,
                    line1: "".to_string(),
                    line2: "".to_string(),
                    city: "".to_string(),
                    country: get_country_from_phone_number(&phone_number).unwrap(),
                },
            }
        })
        .collect();

    export_to_json(&persons, "persons.json").expect("Failed to export `persons` to json");
}
