//! The Java formatting container layer: construction helpers, the format
//! context, comment re-association, and separated-list layout.
//!
//! Java syntax printing lives in the grammar-family modules beside this one.

pub mod builders;
pub mod comments;
pub mod context;
pub mod formatter_java;
pub mod javadoc;
pub mod prelude;
pub mod separated;
pub mod trivia;

pub use self::comments::Comments;
pub use self::context::JavaFormatContext;
pub use self::formatter_java::{JavaFormatter, JavaFormatterExt};
