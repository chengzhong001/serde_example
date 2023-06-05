
fn main(){
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