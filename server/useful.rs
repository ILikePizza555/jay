use std::fmt::Display;

use uuid::Uuid;

/// Converts a `Uuid` into a `String`.
/// This is primarily used by Tabled trait implementations for models.
pub fn format_table_uuid(uuid: &Uuid) -> String {
    String::from(uuid.to_hyphenated().encode_upper(&mut Uuid::encode_buffer()))
}

/// Converts an `Option` into a `String`.
/// This is primarily used by Tabled trait implementations for models.
pub fn format_table_option<T>(opt: &Option<T>, op: impl FnOnce(&T) -> String) -> String {
    opt.as_ref().map(op).unwrap_or(String::from("-"))
}

/// Converts an `Option` containing a type that implements `Display` into a `String`.
/// This is primarily used by Tabled trait implementations for models.
pub fn format_table_option_display<T: Display>(opt: &Option<T>) -> String {
    format_table_option(opt, |t| format!("{}", t))
}