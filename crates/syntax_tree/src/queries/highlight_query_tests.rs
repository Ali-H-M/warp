use super::*;
use crate::DARK_MODERN;

#[test]
fn keyword_control_takes_priority_over_keyword() {
    let color = convert_capture_name_to_color("keyword.control", &DARK_MODERN);
    assert_eq!(color, Some(DARK_MODERN.keyword_control_color));
}

#[test]
fn plain_keyword_falls_back_to_first_segment() {
    let color = convert_capture_name_to_color("keyword.operator", &DARK_MODERN);
    assert_eq!(color, Some(DARK_MODERN.keyword_color));
}

#[test]
fn function_method_takes_priority_over_function() {
    let color = convert_capture_name_to_color("function.method", &DARK_MODERN);
    assert_eq!(color, Some(DARK_MODERN.method_color));
}

#[test]
fn string_escape_takes_priority_over_string() {
    let color = convert_capture_name_to_color("string.escape", &DARK_MODERN);
    assert_eq!(color, Some(DARK_MODERN.escape_color));
}

#[test]
fn variable_parameter_takes_priority_over_variable() {
    let color = convert_capture_name_to_color("variable.parameter", &DARK_MODERN);
    assert_eq!(color, Some(DARK_MODERN.parameter_color));
}

#[test]
fn unmatched_capture_name_returns_none() {
    assert_eq!(convert_capture_name_to_color("clownshoes", &DARK_MODERN), None);
}
