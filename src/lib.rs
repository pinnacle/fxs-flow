#![feature(decl_macro)]
#![feature(never_type)]

pub mod unwrap_or_match;
pub mod unwrap_or;
pub mod unwrappable;

pub use unwrap_or::unwrap_or;
pub use unwrap_or_match::unwrap_or_match;
pub use unwrappable::Unwrappable;
