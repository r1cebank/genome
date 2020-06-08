mod marker;

pub struct Gene {
    pub markers: marker::Marker
}

impl Gene {
    pub fn new() -> Gene {
        Gene {
            markers: marker::Marker::new()
        }
    }
}
