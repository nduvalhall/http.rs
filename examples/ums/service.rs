use crate::{context::Context, user::User};

pub fn create_user(context: &mut Context, user: User) {
    context.database.create_user(user);
}

pub fn print_users(context: &mut Context, _: ()) {
    println!("Users: {:?}", context.database.get_users());
}
