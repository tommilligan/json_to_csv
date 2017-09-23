extern crate spectral;
use spectral::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
    #[test]
    fn read_json_file_example() {
        let data = read_json_file(String::from("tests/example.json"));
        assert_that(&"spam").is_equal_to(&"spam");
    }
}

extern crate serde_json;
use std::fs::File;
use std::io::Read;

pub fn read_json_file(filename: String) -> serde_json::Value {
    let mut file = File::open(filename).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    println!("Read JSON from file: {:?}", &data);
    serde_json::from_str(&data).unwrap()
}

