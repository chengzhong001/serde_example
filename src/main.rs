use serde::Deserialize;
use serde::Serialize;
use std;

#[derive(Serialize, Deserialize, Debug)]
struct Bill {
    amount: i32,
    tax_percent: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    phone: String,
    bills: Vec<Bill>,
}

fn modify_json() {
    let json_data = std::fs::read_to_string("data.json").unwrap();
    let mut data: serde_json::Value = serde_json::from_str(json_data.as_str()).unwrap();
    println!("before: {}", data.to_string());
    data["car"] = serde_json::Value::String("Audi".to_string());
    println!("then: {}", data.to_string());
    let mut map_value = serde_json::Map::new();
    map_value.insert(
        "color".to_string(),
        serde_json::Value::String("blue".to_string()),
    );
    map_value.insert(
        "year".to_string(),
        serde_json::Value::String("1900".to_string()),
    );

    data["car"] = serde_json::Value::Object(map_value);
    let new_data = serde_json::to_string_pretty(&data).unwrap();
    
    println!("after: {}", new_data.to_string());
    std::fs::write("new_data.json", new_data).expect("msg");
}

fn main() {
    let person = Person {
        name: "bob".to_string(),
        age: 45,
        phone: "400100".to_string(),
        bills: vec![
            Bill {
                amount: 78,
                tax_percent: 0.3,
            },
            Bill {
                amount: 101,
                tax_percent: 0.2,
            },
        ],
    };

    let json_str = serde_json::to_string_pretty(&person).unwrap();
    let yaml_str = serde_yaml::to_string(&person).unwrap();

    println!("json: \n{json_str}");
    println!("yaml: \n{yaml_str}");

    let person_json: Person = serde_json::from_str(json_str.as_str()).unwrap();
    let person_yaml: Person = serde_yaml::from_str(yaml_str.as_str()).unwrap();

    println!("{:?}", person_json);
    println!("{:?}", person_yaml);

    modify_json();
}
