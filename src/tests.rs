#[test]
fn test_01() {
    let test_string = String::from("teststststring");
    assert_eq!(super::string_filter(&test_string), "stststst");
}

#[test]
fn test_02() { 
    let test_string = String::from("tesssssstststring");
    assert_eq!(super::string_filter(&test_string), "sssssststst");
}

#[test]
fn test_03() { 
    let test_string = String::from("te");
    assert_eq!(super::string_filter(&test_string), "te");
}

#[test]
fn test_04() { 
    let test_string = String::from("t");
    assert_eq!(super::string_filter(&test_string), "t");
}

#[test]
fn test_05() { 
    let test_string = String::from("");
    assert_eq!(super::string_filter(&test_string), "");
}

#[test]
fn test_06() { 
    let test_string = String::from("tessssssTststring");
    assert_eq!(super::string_filter(&test_string), "ssssssTs");
}

#[test]
fn test_07() { 
    let test_string = String::from("@$@##%%%%$$%#^#@$@");
    assert_eq!(super::string_filter(&test_string), "%%%%$$%");
}
