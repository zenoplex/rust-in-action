use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longtitude: f64,
    sample: Vec<String>,
}

impl City {
    fn new(name: &str, population: usize, latitude: f64, longtitude: f64) -> City {
        City {
            name: String::from(name),
            population: population,
            latitude: latitude,
            longtitude: longtitude,
            sample: vec![String::from("abc"); 3],
        }
    }
}

fn main() {
    let calabar = City::new("Calabar", 470_000, 4.95, 8.33);
    let as_json = serde_json::to_string(&calabar).unwrap();
    let as_cbor = serde_cbor::to_vec(&calabar).unwrap();
    let as_bincode = bincode::serialize(&calabar).unwrap();

    println!("json: {}", &as_json);
    println!("cbor: {:?}", &as_cbor);
    println!("cbor UTF-8: {:?}", String::from_utf8_lossy(&as_cbor));
    println!("bincode: {:?}", &as_bincode);
    println!("bincode UTF-8: {:?}", String::from_utf8_lossy(&as_bincode));

}
