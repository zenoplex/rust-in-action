use serde_json::json;

fn main() {
    let capitals = json!({
        "Cook Island": "Avarua",
        "Fiji":"Suva",
        "Kiribati":"South Tarawa",
        "Niue":"Alofi",
        "Tonga":"Nuku'alofa",
        "Tuvalu":"Funafuti"
    });

    let a = &capitals["Tonga"];
    let b = &capitals["Fiji"];
    let c = capitals.get("Niue").unwrap();

    println!("Capital of Tonga is: {}", a);
    println!("Capital of Fiji is: {}", b);
    println!("Capital of Niue is: {}", c);
}
