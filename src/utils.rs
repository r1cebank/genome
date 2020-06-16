use std::str;

pub fn f32_to_string(data: f32) -> String {
    data.to_be_bytes()
        .iter()
        .map(|val| format!("{:0>2x}", val))
        .collect()
}
pub fn u16_to_string(data: u16) -> String {
    data.to_be_bytes()
        .iter()
        .map(|val| format!("{:0>2x}", val))
        .collect()
}
pub fn partition_string<'a>(data: &'a String, size: usize) -> Vec<&'a str> {
    data.as_bytes()
        .chunks(size)
        .map(str::from_utf8)
        .collect::<Result<Vec<&'a str>, _>>()
        .unwrap()
}

pub fn partition_str<'a>(data: &'a str, size: usize) -> Vec<&'a str> {
    data.as_bytes()
        .chunks(size)
        .map(str::from_utf8)
        .collect::<Result<Vec<&'a str>, _>>()
        .unwrap()
}
