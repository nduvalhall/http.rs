use std::collections::HashMap;

/// A simple pipe-delimited key-value protocol.
///
/// # Format
/// ```text
/// key=value|key=value|...
/// ```
///
/// # Rules
/// - All values are strings; type casting is the caller's responsibility
/// - Whitespace is ignored
/// - `|` is reserved and must not appear in keys or values
/// - `=` is reserved and must not appear in keys or values
pub struct Pipe {
    values: HashMap<String, String>,
}

impl Pipe {
    pub fn new(values: HashMap<String, String>) -> Self {
        Pipe { values }
    }

    pub fn from_str(str: &str) -> Option<Self> {
        let pairs = str.split('|');

        let mut values = HashMap::new();
        for pair in pairs {
            let (key, value) = pair.split_once('=')?;
            values.insert(key.to_string(), value.to_string());
        }
        Some(Self::new(values))
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        Self::from_str(std::str::from_utf8(bytes).ok()?)
    }

    pub fn to_string(&self) -> String {
        let mut pairs = Vec::new();
        for (key, value) in self.values.iter() {
            pairs.push(format!("{}={}", key, value));
        }
        pairs.join("|")
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.values.get(key).map(|s| s.to_string())
    }
}
