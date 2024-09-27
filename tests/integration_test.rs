use gnparser::{parse_ary_to_string, parse_to_string};

#[test]
fn test_parse_to_string() {
    let result = parse_to_string("Homo sapiens", "compact", true, false, false).unwrap();
    println!("Parsed result: {}", result);
    assert!(result.contains("Homo sapiens"));
}

#[test]
fn test_parse_ary_to_string() {
    let names = vec!["Homo sapiens", "Pan troglodytes"];
    let result = parse_ary_to_string(&names, "compact", true, false, false).unwrap();
    println!("Parsed array result: {}", result);
    assert!(result.contains("Homo sapiens") && result.contains("Pan troglodytes"));
}
