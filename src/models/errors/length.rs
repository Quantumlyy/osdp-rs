use std::fmt;

#[derive(Debug, Clone)]
pub struct InvalidLengthError {
    pub minimum: usize,
    pub maximum: usize,
}

impl InvalidLengthError {
    pub fn new(minimum: usize, maximum: usize) -> Self {
        Self { minimum, maximum }
    }

    pub fn new_minimum(minimum: usize) -> Self {
        Self::new(minimum, usize::MAX)
    }

    pub fn new_maximum(maximum: usize) -> Self {
        Self::new(0, maximum)
    }
}

impl Default for InvalidLengthError {
    fn default() -> Self {
        Self::new(0, usize::MAX)
    }
}

impl fmt::Display for InvalidLengthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid length: {}..{}", self.minimum, self.maximum)
    }
}
