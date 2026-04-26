use nailbot::bot::handlers::user::{redact_phone, validate_name, validate_phone};

#[test]
fn test_name_validation() {
    assert!(validate_name("Alice Bob"));
    assert!(!validate_name(""));
}

#[test]
fn test_phone_validation() {
    assert!(validate_phone("+1 234-567-8901"));
    assert!(!validate_phone("abc"));
}

#[test]
fn test_phone_redaction() {
    assert_eq!(redact_phone("+12345678901"), "+***-***-***-8901");
}
