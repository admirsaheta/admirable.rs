mod pointer;
mod bearer;
mod char_type;

use crate::{iterator::pointer::Pointer, Error};

pub use bearer::Bearer;
pub use char_type::CharType;

pub struct Iterator {
    bearers: Vec<Bearer>,
    errors: Vec<Error>
}

