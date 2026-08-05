use crate::core::ext::Rslt;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("")]
pub struct UserCancelled;

impl UserCancelled {
    pub fn to_rslt<T>(self) -> Rslt<T> {
        Err(UserCancelled.into())
    }
}
