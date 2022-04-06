#![feature(test)]
extern crate test;

use fake_it::person::Person;
use test::Bencher;

#[bench]
fn bench(b: &mut Bencher) {
    b.iter(|| create_100_random_person_names());
}

fn create_100_random_person_names() {
    for _ in 0..100 {
        let _names = Person::gen_rand_fname();
        let _surnames = Person::gen_rand_lname();
    }
}
