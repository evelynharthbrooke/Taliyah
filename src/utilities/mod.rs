pub mod built_info;
pub mod database;
pub mod geo_utils;
pub mod git_utils;

pub fn format_int(i: usize) -> String {
    let mut s = String::new();
    let i_str = i.to_string();
    let a = i_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, ',');
        }
        s.insert(0, val);
    }
    return s;
}
