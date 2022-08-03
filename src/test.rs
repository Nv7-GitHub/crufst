use super::*;

#[test]
fn test_db() {
    let mut tab = Table::new("test.db", vec![
        Column{typ: ColType::INT, name: "ID".to_string(), index: true},
        Column{typ: ColType::STRING, name: "Name".to_string(), index: true},
    ]).unwrap();
    tab.insert(vec![Value::INT(0), Value::STRING("HI".to_string())]).unwrap();
}