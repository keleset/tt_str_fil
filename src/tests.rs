#[test]
fn test_01_lat() {
    let test_string = String::from("teststststring");
    assert_eq!(super::string_filter(&test_string), "stststst");
}

#[test]
fn test_02_lat() {
    let test_string = String::from("tesssssstststring");
    assert_eq!(super::string_filter(&test_string), "sssssststst");
}

#[test]
fn test_03_short() {
    let test_string = String::from("te");
    assert_eq!(super::string_filter(&test_string), "te");
}

#[test]
fn test_04_short() {
    let test_string = String::from("t");
    assert_eq!(super::string_filter(&test_string), "t");
}

#[test]
fn test_05_empty() {
    let test_string = String::from("");
    assert_eq!(super::string_filter(&test_string), "");
}

#[test]
fn test_06_upper() {
    let test_string = String::from("tessssssTststring");
    assert_eq!(super::string_filter(&test_string), "ssssssTs");
}

#[test]
fn test_07_special() {
    let test_string = String::from("@$@##%%%%$$%#^#@$@");
    assert_eq!(super::string_filter(&test_string), "%%%%$$%");
}
#[test]
fn test_08_cyr() {
    let test_string = String::from("тессстстстринг");
    assert_eq!(super::string_filter(&test_string), "ссстстст");
}
#[test]
fn test_09_mix() {
    let test_string = String::from("те虎131313131313ринг");
    assert_eq!(super::string_filter(&test_string), "131313131313");
}
#[test]
fn test_10_mix() {
    let test_string = String::from("123е虎675777е虎е虎е虎е虎е虎е虎");
    assert_eq!(
        super::string_filter(&test_string),
        "е虎е虎е虎е虎е虎е虎"
    );
}
#[test]
fn test_11_mix() {
    let test_string = String::from("e虎e虎e虎e虎e虎e虎123е虎675777");
    assert_eq!(
        super::string_filter(&test_string),
        "e虎e虎e虎e虎e虎e虎"
    );
}
