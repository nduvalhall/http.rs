use crate::{
    context::Context,
    error::Error,
    user::{RaiseError, User},
};

pub fn create_user(context: &mut Context, user: User) -> Result<(), Error> {
    context.database.create_user(user);
    Ok(())
}

pub fn print_users(context: &mut Context, _: ()) -> Result<(), Error> {
    println!("Users: {:?}", context.database.get_users());
    Ok(())
}

pub fn raise_error(_: &mut Context, error: RaiseError) -> Result<(), Error> {
    Err(Error::UnknownError(error.error))
}
