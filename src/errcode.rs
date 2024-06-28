//! Bayer error codes.

pub type BayerResult<T> = Result<T, BayerError>;

#[derive(Debug)]
pub enum BayerError {
    // Generic failure.  Please try to make something more meaningful.
    NoGood,
    WrongResolution,
    WrongDepth,
    Io(std::io::Error),
}

impl std::fmt::Display for BayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           Self::NoGood => write!(f, "No good"),
           Self::WrongResolution => write!(f, "Wrong resolution"),
           Self::WrongDepth => write!(f, "Wrong depth"),
           Self::Io(e) => write!(f, "IO error: {}", e),
       }
    }
}

impl From<std::io::Error> for BayerError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl std::error::Error for BayerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::NoGood => None,
            Self::WrongResolution => None,
            Self::WrongDepth => None,
            Self::Io(e) => Some(e),
        }
    }
}
