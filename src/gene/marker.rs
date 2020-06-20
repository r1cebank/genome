use crate::utils;
use rand::prelude::*;
use rand_distr::StandardNormal;

#[derive(Copy, Clone)]
pub struct Marker {
    pub value: f32,
}

impl Marker {
    pub fn new() -> Marker {
        Marker {
            value: thread_rng().sample(StandardNormal),
        }
    }
    pub fn to_string(&self) -> String {
        utils::f32_to_string(self.value)
    }
}

impl std::convert::From<Marker> for String {
    fn from(marker: Marker) -> String {
        let byte_marker = marker.value.to_be_bytes();

        byte_marker
            .iter()
            .map(|val| format!("{:0>2x}", val))
            .collect()
    }
}

impl std::convert::From<f32> for Marker {
    fn from(marker: f32) -> Marker {
        Marker { value: marker }
    }
}

impl std::convert::From<String> for Marker {
    fn from(marker: String) -> Marker {
        Marker {
            value: utils::f32_from_str(&marker),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_be_converted_and_back() {
        let marker = Marker::new();
        let string_value: String = marker.into();
        let restored_marker: Marker = string_value.into();
        assert_eq!(marker.value, restored_marker.value);
    }
}
