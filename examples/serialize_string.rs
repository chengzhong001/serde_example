use serde::Deserialize;
use serde::Serialize;


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

}