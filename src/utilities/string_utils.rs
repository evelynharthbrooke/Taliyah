/// Capitalizes the first letter of a str.
pub fn capitalize_first(string: &str) -> String {
    let mut c = string.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}
