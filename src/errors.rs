/// Represents errors that can occur while manipulating posets, such as when the partial order is
/// not a valid partial order.
#[derive(Debug, PartialEq, Eq)]
pub enum PosetError {
    /// Indicates that the poset has no maxima, when it should.
    NoMaxima,
    /// Indicates that the poset has no minima, when it should.
    NoMinima,
}

impl std::fmt::Display for PosetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PosetError::NoMaxima => write!(f, "non-empty poset should have a maximal element"),
            PosetError::NoMinima => write!(f, "non-empty poset should have a minimal element"),
        }
    }
}
