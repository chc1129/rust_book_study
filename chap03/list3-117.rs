// Opthon
pub fn and_then<U, F>(self, f: F) -> Option<U>
where
    FnOnce(T) -> Option<U>,

// Result
pub fn and_then<U, F>(self, op: F) -> Result<U, E>
where
    FnOnce(T) -> U,
