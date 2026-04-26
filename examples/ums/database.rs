use crate::user::User;

pub struct Database {
    users: Vec<User>,
}

impl Database {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }

    pub fn create_user(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn get_users(&self) -> &Vec<User> {
        &self.users
    }
}
