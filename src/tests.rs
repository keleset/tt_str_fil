#[test]
fn test_lat_01() {
    let test_string = String::from("teststststring");
    assert_eq!(super::string_filter(&test_string), "stststst");
}

#[test]
fn test_lat_02() { 
    let test_string = String::from("tesssssstststring");
    assert_eq!(super::string_filter(&test_string), "sssssststst");
}

#[test]
fn test_short_03() { 
    let test_string = String::from("te");
    assert_eq!(super::string_filter(&test_string), "te");
}

#[test]
fn test_short_04() { 
    let test_string = String::from("t");
    assert_eq!(super::string_filter(&test_string), "t");
}

#[test]
fn test_empty_05() { 
    let test_string = String::from("");
    assert_eq!(super::string_filter(&test_string), "");
}

#[test]
fn test_upper_06() { 
    let test_string = String::from("tessssssTststring");
    assert_eq!(super::string_filter(&test_string), "ssssssTs");
}

#[test]
fn test_special_07() { 
    let test_string = String::from("@$@##%%%%$$%#^#@$@");
    assert_eq!(super::string_filter(&test_string), "%%%%$$%");
}
#[test]
fn test_cyr_08() { 
    let test_string = String::from("тессстстстринг");
    assert_eq!(super::string_filter(&test_string), "ссстстст");
}
#[test]
fn test_mix_09() { 
    let test_string = String::from("те虎с虎с虎с虎тстстринг");
    assert_eq!(super::string_filter(&test_string), "虎с虎с虎с虎");
}
