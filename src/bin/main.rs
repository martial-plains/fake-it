use fake_it::{person::Person, utils::export_to_html};

fn main() {
    let mut people: Vec<Person> = Vec::new();

    for _ in 0..100 {
        let person = Person::default();
        people.push(person);
    }

    export_to_html(&people, "subscribers.html").expect("Failed to export to html");
}
