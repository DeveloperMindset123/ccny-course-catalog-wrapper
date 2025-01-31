//! Utility functions and helpers
use std::collections::HashMap;

/// String interning is a method of storing only one copy of each distinct string value, which must be immutable.
/// String interning for memory optimization
pub struct StringInterner {
    pub strings : Vec<String>,                  // serves as the key for the lookup table
    pub lookup : HashMap<String, u32>           // stores string and ID corresponding to it
}

impl StringInterner {
    /// Creates a new StringInterner
    pub fn new() -> Self {
        StringInterner {
            strings : Vec::new(),
            lookup : HashMap::new(),
        }
    }

    /// Interns a string and returns its ID
    pub fn intern(&mut self, s : String) -> u32 {
        // TODO : implement this
        12      // placeholder val
    }
}




