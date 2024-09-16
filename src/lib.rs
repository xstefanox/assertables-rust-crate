//! # Assertables: Rust crate of assert macros for testing
//!
//! The `assertables` Rust crate provides many assert macros to improve your
//! compile-time tests and run-time reliability.
//!
//! * Crate: [https://crates.io/crates/assertables](https://crates.io/crates/assertables)
//! * Docs: [https://docs.rs/assertables/](https://docs.rs/assertables/)
//! * Repo: [https://github.com/sixarm/assertables-rust-crate/](https://github.com/sixarm/assertables-rust-crate/)
//! * Contact: [joel@joelparkerhenderson.com](mailto:joel@joelparkerhenderson.com)
//!
//!
//! ## Introduction
//!
//! The Rust programming language provides a few built-in assert macros to test code:
//!
//! The Rust programming language provides assert macros to test code:
//!
//! * [`assert!()`](https://doc.rust-lang.org/std/macro.assert.html)
//! * [`assert_eq!(a, b)`](https://doc.rust-lang.org/std/macro.assert_eq.html)
//! * [`assert_ne!(a, b)`](https://doc.rust-lang.org/std/macro.assert_ne.html)
//!
//! The assertables crate provides many more, so you can write smarter tests.
//!
//! For values:
//!
//! * [`assert_lt!(a, b)`](macro@crate::assert_lt) `// less than`
//! * [`assert_le!(a, b)`](macro@crate::assert_le) `// less than or equal to`
//! * [`assert_gt!(a, b)`](macro@crate::assert_gt) `// greater than`
//! * [`assert_ge!(a, b)`](macro@crate::assert_ge) `// greater than or equal to`
//!
//! For strings:
//!
//! * [`assert_starts_with!(a, b)`](macro@crate::assert_starts_with)
//! * [`assert_ends_with!(a, b)`](macro@crate::assert_ends_with)
//!
//! For matching:
//!
//! * [`assert_contains!(a, b)`](macro@crate::assert_contains)
//! * [`assert_is_match!(a, b)`](macro@crate::assert_is_match)
//!
//! For infix operators:
//!
//! * [`assert_infix!(a == b)`](macro@crate::assert_infix)
//! * [`assert_infix!(a && b)`](macro@crate::assert_infix)
//!
//! For numbers:
//!
//! * [`assert_in_delta!(a, b, delta)`](macro@crate::assert_in_delta)
//! * [`assert_in_epsilon!(a, b, epsilon)`](macro@crate::assert_in_epsilon)
//!
//! For results:
//!
//! * [`assert_result_ok!(a)`](macro@crate::assert_result_ok)
//! * [`assert_result_ok_eq!(a)`](macro@crate::assert_result_ok_eq)
//! * [`assert_result_ok_ne!(a)`](macro@crate::assert_result_ok_ne)
//! * [`assert_result_err!(a)`](macro@crate::assert_result_err)
//!
//! For options:
//!
//! * [`assert_option_some!(a)`](macro@crate::assert_option_some)
//! * [`assert_option_some_eq!(a)`](macro@crate::assert_option_some_eq)
//! * [`assert_option_some_ne!(a)`](macro@crate::assert_option_some_ne)
//! * [`assert_option_none!(a)`](macro@crate::assert_option_none)
//!
//! For polls:
//!
//! * [`assert_poll_ready!(a)`](macro@crate::assert_poll_ready)
//! * [`assert_poll_ready_eq!(a, b)`](macro@crate::assert_poll_ready_eq)
//! * [`assert_poll_ready_ne!(a, b)`](macro@crate::assert_poll_ready_ne)
//! * [`assert_poll_pending!(a)`](macro@crate::assert_poll_pending)
//!
//! For collections such as arrays, vectors, maps, sets:
//!
//! * [`assert_set_subset!(collection1, collection2)`](macro@crate::assert_set_subset)
//! * [`assert_set_disjoint!(collection1, collection2)`](macro@crate::assert_set_disjoint)
//!
//! For file system paths and input/output readers:
//!
//! * [`assert_fs_read_to_string_eq!(path1, path2)`](macro@crate::assert_fs_read_to_string_eq)
//! * [`assert_io_read_to_string_eq!(reader1, reader2)`](macro@crate::assert_io_read_to_string_eq)
//!
//! For command capture of standard output and standard error:
//!
//! * [`assert_command_stdout_eq!(command1, command2)`](macro@crate::assert_command_stdout_eq)
//! * [`assert_program_args_stdout_eq!(program1, args1, program2, args2)`](macro@crate::assert_program_args_stdout_eq);
//!
//! There are many more macros that are grouped into modules.
//!
//! Modules for enums:
//!
//! * [`assert_option`](module@crate::assert_option) for `Option` {`Some`, `None`}
//! * [`assert_result`](module@crate::assert_result) for `Result` {`Ok`, `Err`}
//! * [`assert_poll`](module@crate::assert_poll) for `Poll` {`Ready`, `Pending`}
//!
//! Modules for collections, such as arrays, vectors, lists, maps:
//!
//! * [`assert_set`](module@crate::assert_set) for set collections
//! * [`assert_bag`](module@crate::assert_bag) for bag collections
//!
//! Modules for functions:
//!
//! * [`assert_fn`](module@crate::assert_fn) for functions in general.
//! * [`assert_fn_ok`](module@crate::assert_fn_ok) for functions that return Result::Ok.
//! * [`assert_fn_err`](module@crate::assert_fn_err) for functions that return Result::Err.
//!
//! Modules for readers:
//!
//! * [`assert_fs_read_to_string`](module@crate::assert_fs_read_to_string) for file system path contents.
//! * [`assert_io_read_to_string`](module@crate::assert_io_read_to_string) for input/output reader streams.
//!
//! Modules for external calls:
//!
//! * [`assert_command`](module@crate::assert_command) for commands and their stdout & stderr.
//! * [`assert_program_args`](module@crate::assert_program_args) for programs with args and their stdout & stderr.
//!
//!
//! ### Benefits
//!
//! * Your tests are more purposeful and powerful. This helps your code be more
//! reliable.
//!
//! * Your assert failures provide more information. This helps you
//! troubleshoot faster.
//!
//! * You gain runtime asserts. This helps you with validations and
//! verifications.
//!
//!
//! ### Features
//!
//! * Easy to use: each macro is well-documented with runnable examples and
//! tests.
//!
//! * Zero overhead: if you don't use a macro, then it's never compiled into
//! your code.
//!
//! * Zero dependencies: the crate has no release dependencies, and just a short list of development dependencies.
//!
//!
//! ## Forms
//!
//!
//! ### Forms for panic versus error
//!
//! All the assert macros have 3 forms for different purposes:
//!
//! * Panic form for typical tests.
//! * Debug form for debugging runtimes.
//! * Result form for runtime checks, verifications, validations, etc.
//!
//! Examples:
//!
//! * [`assert_starts_with!(a, b)`](https://docs.rs/assertables/latest/assertables/macro.assert_starts_with.html) // panic!
//! * [`debug_assert_starts_with!(a, b)`](https://docs.rs/assertables/latest/assertables/macro.debug_assert_starts_with.html) // panic! in debug mode
//! * [`assert_starts_with_as_result!(a, b)`](https://docs.rs/assertables/latest/assertables/macro.assert_starts_with_as_result.html) // return Ok or Err
//!
//!
//! ### Forms for messages
//!
//! All the assert macros have 2 forms for messages.
//!
//! * Default message form.
//! * Custom message form.
//!
//! Examples:
//!
//! * [`assert_starts_with!(a, b)`](https://docs.rs/assertables/latest/assertables/macro.assert_starts_with.html)
//! * [`assert_starts_with!(a, b, "Your custom message here")`](https://docs.rs/assertables/latest/assertables/macro.assert_starts_with.html)
//!
//!
//! ### Forms for other versus expression
//!
//! Many of the assert macros have 2 forms for comparing left hand side and right hand side.
//!
//! * Comparing a LHS item to a RHS other of the same type.
//! * Comparing a LHS item to a RHS expression.
//!
//! Examples:
//!
//! * [`assert_io_read_to_string_eq!(reader1, reader2)`](https://docs.rs/assertables/latest/assertables/macro.assert_io_read_to_string_eq.html)
//! * [`assert_io_read_to_string_eq_expr!(reader, expr)`](https://docs.rs/assertables/latest/assertables/macro.assert_io_read_to_string_eq_expr.html)
//!
//!
//! ## Tracking
//!
//! * Package: assertables-rust-crate
//! * Version: 8.6.0
//! * Created: 2021-03-30T15:47:49Z
//! * Updated: 2024-09-18T03:02:31Z
//! * License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
//! * Contact: Joel Parker Henderson (joel@sixarm.com)

// Assert truth
pub mod assert; // (in addition to what's provided by Rust `std`)

// Assert value comparison
pub mod assert_eq; // (in addition to what's provided by Rust `std`)
pub mod assert_ge;
pub mod assert_gt;
pub mod assert_le;
pub mod assert_lt;
pub mod assert_ne;

// Assert value nearness
pub mod assert_in_delta;
pub mod assert_in_epsilon;
// Assert value matching
pub mod assert_contains;
pub mod assert_ends_with;
pub mod assert_is_match;
pub mod assert_not_contains;
pub mod assert_not_ends_with;
pub mod assert_not_match;
pub mod assert_not_starts_with;
pub mod assert_starts_with;

// For maybes
pub mod assert_result;
pub mod assert_option;
pub mod assert_poll;

// For collections
pub mod assert_set;
pub mod assert_bag;

// For functions
pub mod assert_fn;
pub mod assert_fn_ok;
pub mod assert_fn_err;

// For reading
pub mod assert_fs_read_to_string;
pub mod assert_io_read_to_string;

// For externals
pub mod assert_command;
pub mod assert_program_args;

// Experimental - work in progress - unsupported
pub mod assert_infix;