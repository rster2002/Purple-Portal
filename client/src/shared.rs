use crate::prelude::Error;

#[macro_export]
macro_rules! wrap_ws_error {
    ($name:expr) => {
        match $name {
            Ok(r) => Ok(r),
            Err(e) => Err(Error::WsClientError(e.to_string())),
        }
    };
}
