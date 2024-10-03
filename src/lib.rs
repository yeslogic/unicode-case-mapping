//! Map a character to its lowercase, uppercase, or titlecase equivalent.
//!
//! ### Example
//!
//! ```
//! use std::num::NonZeroU32;
//!
//! // U+0307 is COMBINING DOT ABOVE
//! assert_eq!(unicode_case_mapping::to_lowercase('İ'), ['i' as u32, 0x0307]);
//! assert_eq!(unicode_case_mapping::to_lowercase('ß'), [0; 2]);
//! assert_eq!(unicode_case_mapping::to_uppercase('ß'), ['S' as u32, 'S' as u32, 0]);
//! assert_eq!(unicode_case_mapping::to_titlecase('ß'), ['S' as u32, 's' as u32, 0]);
//! assert_eq!(unicode_case_mapping::to_titlecase('-'), [0; 3]);
//! assert_eq!(unicode_case_mapping::case_folded('I'), NonZeroU32::new('i' as u32));
//! assert_eq!(unicode_case_mapping::case_folded('ß'), None);
//! assert_eq!(unicode_case_mapping::case_folded('ẞ'), NonZeroU32::new('ß' as u32));
//! ```

mod case_folding_simple;
mod case_mapping;
mod tables;
pub use case_mapping::{case_folded, to_lowercase, to_titlecase, to_uppercase};

/// The version of [Unicode](http://www.unicode.org/)
/// that this version of unicode-case-mapping was generated from.
pub const UNICODE_VERSION: (u64, u64, u64) = (16, 0, 0);
