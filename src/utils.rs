use arrayvec::ArrayVec;
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

pub fn u16_from_str<'a>(data: &'a str) -> u16 {
    let decoded_bytes: ArrayVec<_> = partition_str(data, 2)
        .iter()
        .map(|c| u8::from_str_radix(c, 16).unwrap())
        .collect::<ArrayVec<_>>();
    let decoded_bytes_array: [u8; 2] = decoded_bytes.into_inner().unwrap();
    u16::from_be_bytes(decoded_bytes_array)
}

pub fn f32_from_str<'a>(data: &'a str) -> f32 {
    let decoded_bytes: ArrayVec<_> = partition_str(data, 2)
        .iter()
        .map(|c| u8::from_str_radix(c, 16).unwrap())
        .collect::<ArrayVec<_>>();
    let decoded_bytes_array: [u8; 4] = decoded_bytes.into_inner().unwrap();
    f32::from_be_bytes(decoded_bytes_array)
}
