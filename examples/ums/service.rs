use crate::{
    context::Context,
    error::UMSError,
    user::{RaiseError, User},
};

pub fn create_user(context: &mut Context, user: User) -> Result<(), UMSError> {
    context.database.create_user(user);
    Ok(())
}

pub fn print_users(context: &mut Context, _: ()) -> Result<(), UMSError> {
    println!("Users: {:?}", context.database.get_users());
    Ok(())
}

pub fn raise_error(_: &mut Context, error: RaiseError) -> Result<(), UMSError> {
    Err(UMSError::UnknownError(error.error))
}
