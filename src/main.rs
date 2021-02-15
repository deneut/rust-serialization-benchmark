#![feature(test)]
extern crate serde_json;
mod data;

use data::ComplexRF;

pub fn test_many(test_object: Vec<&ComplexRF>, limit: Option<usize>, print_output: bool) {
    for _ in 0..limit.unwrap_or(1000) {
        let json_string = serde_json::to_string(&test_object);
        if print_output {
            println!("Many: {:?}", json_string);
        }
    }
}

pub fn test_one(test_object: &ComplexRF, limit: Option<usize>, print_output: bool) {
    for _ in 0..limit.unwrap_or(1000) {
        let json_string = serde_json::to_string(&test_object);
        if print_output {
            println!("One: {:?}", json_string);
        }
    }
}

fn main() {
    let test_object = ComplexRF::new();

    test_many(vec![&test_object, &test_object], Some(2), true); // Warmup, LOL
    test_many(vec![&test_object, &test_object], None, false);

    test_one(&test_object, Some(2), true); // Warmup
    test_one(&test_object, None, false);
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[bench]
    fn many_objects(b: &mut Bencher) {
        let test_object = ComplexRF::new();
        b.iter(|| test_many(vec![&test_object, &test_object], None, false));
    }
    #[bench]
    fn one_object(b: &mut Bencher) {
        let test_object = ComplexRF::new();
        b.iter(|| test_one(&test_object, None, false));
    }

}
