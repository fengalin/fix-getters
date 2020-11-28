pub mod dir_entry;

pub mod function;
pub use function::{
    try_rename_getter_call, try_rename_getter_def, GetFunction, RenameError, RenameOk, ReturnsBool,
};
