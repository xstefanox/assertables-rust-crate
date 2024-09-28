//! Assert expression is Err(_).
//!
//! Pseudocode:<br>
//! a is Err(_)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! # fn main() {
//! let a: Result<(), i8> = Err(1);
//! assert_err!(a);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_err`](macro@crate::assert_err)
//! * [`assert_err_as_result`](macro@crate::assert_err_as_result)
//! * [`debug_assert_err`](macro@crate::debug_assert_err)

/// Assert expression is Err(_).
///
/// Pseudocode:<br>
/// a is Err(_)
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_err`](macro.assert_err.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_err`](macro@crate::assert_err)
/// * [`assert_err_as_result`](macro@crate::assert_err_as_result)
/// * [`debug_assert_err`](macro@crate::debug_assert_err)
///
#[macro_export]
macro_rules! assert_err_as_result {
    ($a:expr $(,)?) => {{
        match (&$a) {
            a => {
                match (a) {
                    Err(_) => {
                        Ok(())
                    },
                    _ => {
                        Err(format!(
                            concat!(
                                "assertion failed: `assert_err!(a)`\n",
                                "https://docs.rs/assertables/8.10.1/assertables/macro.assert_err.html\n",
                                " a label: `{}`,\n",
                                " a debug: `{:?}`",
                            ),
                            stringify!($a),
                            a
                        ))
                    }
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_assert_err_as_result_x_success() {
        let a: Result<(), i8> = Err(1);
        let result = assert_err_as_result!(a);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_err_as_result_x_failure() {
        let a: Result<(), i8>  = Ok(());
        let result = assert_err_as_result!(a);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_err!(a)`\n",
                "https://docs.rs/assertables/8.10.1/assertables/macro.assert_err.html\n",
                " a label: `a`,\n",
                " a debug: `Ok(())`",
            )
        );
    }
}

/// Assert expression is Err(_).
///
/// Pseudocode:<br>
/// a is Err(_)
///
/// * If true, return `()`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// use assertables::*;
/// # use std::panic;
/// # fn main() {
/// let a: Result<(), i8> = Err(1);
/// assert_err!(a);
///
/// # let result = panic::catch_unwind(|| {
/// let a: Result<(), i8> = Ok(());
/// assert_err!(a);
/// # });
/// // assertion failed: `assert_err!(a)`
/// // https://docs.rs/assertables/8.10.1/assertables/macro.assert_err.html
/// //  a label: `a`,
/// //  a debug: `Ok(())`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_err!(a)`\n",
/// #     "https://docs.rs/assertables/8.10.1/assertables/macro.assert_err.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `Ok(())`",
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_err`](macro@crate::assert_err)
/// * [`assert_err_as_result`](macro@crate::assert_err_as_result)
/// * [`debug_assert_err`](macro@crate::debug_assert_err)
///
#[macro_export]
macro_rules! assert_err {
    ($a:expr $(,)?) => {{
        match $crate::assert_err_as_result!($a) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($a:expr, $($message:tt)+) => {{
        match $crate::assert_err_as_result!($a) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert expression is Err(_).
///
/// Pseudocode:<br>
/// a is Err(_)
///
/// This macro provides the same statements as [`assert_err`](macro.assert_err.html),
/// except this macro's statements are only enabled in non-optimized
/// builds by default. An optimized build will not execute this macro's
/// statements unless `-C debug-assertions` is passed to the compiler.
///
/// This macro is useful for checks that are too expensive to be present
/// in a release build but may be helpful during development.
///
/// The result of expanding this macro is always type checked.
///
/// An unchecked assertion allows a program in an inconsistent state to
/// keep running, which might have unexpected consequences but does not
/// introduce unsafety as long as this only happens in safe code. The
/// performance cost of assertions, however, is not measurable in general.
/// Replacing `assert*!` with `debug_assert*!` is thus only encouraged
/// after thorough profiling, and more importantly, only in safe code!
///
/// This macro is intended to work in a similar way to
/// [`std::debug_assert`](https://doc.rust-lang.org/std/macro.debug_assert.html).
///
/// # Module macros
///
/// * [`assert_err`](macro@crate::assert_err)
/// * [`assert_err`](macro@crate::assert_err)
/// * [`debug_assert_err`](macro@crate::debug_assert_err)
///
#[macro_export]
macro_rules! debug_assert_err {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_err!($($arg)*);
        }
    };
}
