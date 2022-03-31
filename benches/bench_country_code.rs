#![feature(test)]
extern crate test;

use test::Bencher;

use fake_it::utils::get_country_phone_code_by_name;

#[bench]
fn bench_get_usa(b: &mut Bencher) {
    b.iter(|| get_country_phone_code_by_name("United States"));
}
