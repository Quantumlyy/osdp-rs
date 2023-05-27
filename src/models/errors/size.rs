use std::fmt;

#[derive(Debug, Clone)]
pub struct InvalidPacketSizeError {
    pub minimum: usize,
    pub maximum: usize,
    pub received: usize,
}

impl InvalidPacketSizeError {
    pub fn new(minimum: usize, maximum: usize, received: usize) -> Self {
        Self { minimum, maximum, received }
    }

    pub fn new_minimum(minimum: usize, received: usize) -> Self {
        Self::new(minimum, usize::MAX, received)
    }

    pub fn new_maximum(maximum: usize, received: usize) -> Self {
        Self::new(0, maximum, received)
    }
}

impl Default for InvalidPacketSizeError {
    fn default() -> Self {
        Self::new(0, usize::MAX, 0)
    }
}

impl fmt::Display for InvalidPacketSizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid size, expected: {}..{}; received: {}", self.minimum, self.maximum, self.received)
    }
}
