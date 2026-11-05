use inertia::{Inertia, props};

use crate::auth::AuthenticatedUser;
use crate::errors::AppError;

pub async fn index(
    inertia: Inertia,
    _user: AuthenticatedUser,
) -> Result<inertia::Response, AppError> {
    Ok(inertia.render("Dashboard", props! {}).encrypt_history(true))
}
