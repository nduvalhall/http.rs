pub struct Pipe {
    values: Vec<String>,
}

impl Pipe {
    pub fn new(values: Vec<String>) -> Self {
        Pipe { values }
    }

    pub fn from_str(str: &str) -> Self {
        Self::new(str.split('|').map(|s| s.to_string()).collect())
    }

    pub fn from_string(string: &String) -> Self {
        Self::from_str(&string)
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self::from_str(std::str::from_utf8(bytes).unwrap())
    }

    pub fn to_string(&self) -> String {
        self.values.join("|")
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_string().as_bytes().to_vec()
    }

    pub fn get(&self, index: usize) -> Option<&String> {
        self.values.get(index)
    }
}
