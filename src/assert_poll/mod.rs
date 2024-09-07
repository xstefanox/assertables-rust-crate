//! Assert macros for `Poll` {`Ready`, `Pending`}
//!
//! These macros help compare a `Poll` that is either `Ready`, `Pending`.
//!
//! The macros use these capabilities:
//!
//! * implements `.is_ready() -> boolean`
//!
//! * implements `.is_pending() -> boolean`
//!
//! # Macros
//!
//! * [`assert_poll_ready!(a)`](macro@crate::assert_poll_ready) ≈ a.is_ready()
//!
//! * [`assert_poll_pending!(a)`](macro@crate::assert_poll_pending) ≈ a.is_pending()
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! use std::task::Poll;
//!
//!
//! # fn main() {
//! let a: Poll<i8> = Poll::Ready(1);
//! assert_poll_ready!(a);
//! # }
//! ```

pub mod assert_poll_ready;
pub mod assert_poll_pending;
