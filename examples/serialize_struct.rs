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

fn main(){
    let  json_data = std::fs::read_to_string("json_data.json").unwrap();
    let  ymal_data = std::fs::read_to_string("yaml_data.yml").unwrap();

    let person_json: Person = serde_json::from_str(json_data.as_str()).unwrap();
    let person_yaml: Person = serde_yaml::from_str(ymal_data.as_str()).unwrap();

    println!("{:?}", person_json);
    println!("{:?}", person_yaml);


}