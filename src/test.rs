use super::*;

#[test]
fn test_db() {
    let tab = Table::new("test.db", vec![
        Column{typ: ColType::INT, name: "ID".to_string(), index: true},
        Column{typ: ColType::STRING, name: "Name".to_string(), index: true},
    ]).unwrap();
}