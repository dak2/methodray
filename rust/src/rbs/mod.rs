//! RBS type loading and conversion

// Converter is always available (no Ruby FFI dependency)
pub mod converter;
pub use converter::RbsTypeConverter;

// These require Ruby FFI for RBS loading
#[cfg(feature = "ruby-ffi")]
pub mod error;
#[cfg(feature = "ruby-ffi")]
pub mod loader;

#[cfg(feature = "ruby-ffi")]
pub use error::RbsError;
#[cfg(feature = "ruby-ffi")]
pub use loader::{register_rbs_methods, RbsLoader, RbsMethodInfo};
