use crate::database::Database;

pub struct Context {
    pub database: Database,
}

impl Context {
    pub fn new() -> Self {
        Self {
            database: Database::new(),
        }
    }
}
