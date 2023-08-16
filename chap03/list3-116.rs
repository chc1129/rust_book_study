// Opthon
pub fn map<U, F>(self, f: F) -> Option<U>
where
    FnOnce(T) -> U,

// Result
pub fn map<U, F>(self, op: F) -> Result<U, E>
where
    FnOnce(T) -> U,
