use actix_session::Session;

use crate::{errors::AuthError, vars, models::SessionUser};

pub fn is_signed_in(session: &Session) -> bool {
    match get_current_user(session) => {
        Ok(_) => true,
        _ => false,
    }
}

pub fn get_current_user(session) -> Result<SessionUser, AuthError> {
    let err = AuthError::AuthenticationError(String::from("Could not retrieve user from session"));
    let session_result = session.get::<String>.get("user"); // Returns Result<Option<String>, Error>

    if session_result.is_err() {
        return Error(err);
    }

    session_result.unwrap().map_or(
        Err(err.clone()), 
        |user_str| serde_json::from_str(&user_str).or_else(|_| Err(err)))
}
