//! Assert two expressions are Ready(_) and their values are not equal.
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! use std::task::Poll;
//! use std::task::Poll::*;
//! # fn main() {
//! let a: Poll<i8> = Ready(1);
//! let b: Poll<i8> = Ready(2);
//! assert_poll_ready_ne!(a, b);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_poll_ready_ne`](macro@crate::assert_poll_ready_ne)
//! * [`assert_poll_ready_ne_as_result`](macro@crate::assert_poll_ready_ne_as_result)
//! * [`debug_assert_poll_ready_ne`](macro@crate::debug_assert_poll_ready_ne)

/// Assert two expressions are Ready(_) and their values are not equal.
///
/// * If true, return Result `Some(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_poll_ready_ne`](macro.assert_poll_ready_ne.html),
/// except this macro returns a Option, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_poll_ready_ne`](macro@crate::assert_poll_ready_ne)
/// * [`assert_poll_ready_ne_as_result`](macro@crate::assert_poll_ready_ne_as_result)
/// * [`debug_assert_poll_ready_ne`](macro@crate::debug_assert_poll_ready_ne)
///
#[deprecated(note = "Please rename from `assert_poll_ready_ne_as_result` to `assert_ready_ne_as_result` because more developers prefer the shorter name.")]
#[macro_export]
macro_rules! assert_poll_ready_ne_as_result {
    ($a:expr, $b:expr $(,)?) => ({
        match (&$a, &$b) {
            (a, b) => {
                match (a, b) {
                    (Ready(a_inner), Ready(b_inner)) => {
                        if a_inner != b_inner {
                            Ok(())
                        } else {
                            Err(format!(
                                concat!(
                                    "assertion failed: `assert_poll_ready_ne!(a, b)`\n",
                                    "https://docs.rs/assertables/8.7.0/assertables/macro.assert_poll_ready_ne.html\n",
                                    " a label: `{}`,\n",
                                    " a debug: `{:?}`,\n",
                                    " a inner: `{:?}`,\n",
                                    " b label: `{}`,\n",
                                    " b debug: `{:?}`,\n",
                                    " b inner: `{:?}`"
                                ),
                                stringify!($a),
                                a,
                                a_inner,
                                stringify!($b),
                                b,
                                b_inner
                            ))
                        }
                    },
                    _ => {
                        Err(format!(
                            concat!(
                                "assertion failed: `assert_poll_ready_ne!(a, b)`\n",
                                "https://docs.rs/assertables/8.7.0/assertables/macro.assert_poll_ready_ne.html\n",
                                " a label: `{}`,\n",
                                " a debug: `{:?}`,\n",
                                " b label: `{}`,\n",
                                " b debug: `{:?}`",
                            ),
                            stringify!($a),
                            a,
                            stringify!($b),
                            $b
                        ))
                    }
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use std::task::Poll;
    use std::task::Poll::*;

    #[test]
    fn test_assert_poll_ready_ne_as_result_x_success() {
        let a: Poll<i8> = Ready(1);
        let b: Poll<i8> = Ready(2);
        let result = assert_poll_ready_ne_as_result!(a, b);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_assert_poll_ready_ne_as_result_x_failure_because_ne() {
        let a: Poll<i8> = Ready(1);
        let b: Poll<i8> = Ready(1);
        let result = assert_poll_ready_ne_as_result!(a, b);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_poll_ready_ne!(a, b)`\n",
                "https://docs.rs/assertables/8.7.0/assertables/macro.assert_poll_ready_ne.html\n",
                " a label: `a`,\n",
                " a debug: `Ready(1)`,\n",
                " a inner: `1`,\n",
                " b label: `b`,\n",
                " b debug: `Ready(1)`,\n",
                " b inner: `1`",
            )
        );
    }

    #[test]
    fn test_assert_poll_ready_ne_as_result_x_failure_because_not_ready() {
        let a: Poll<i8> = Ready(1);
        let b: Poll<i8> = Pending;
        let result = assert_poll_ready_ne_as_result!(a, b);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_poll_ready_ne!(a, b)`\n",
                "https://docs.rs/assertables/8.7.0/assertables/macro.assert_poll_ready_ne.html\n",
                " a label: `a`,\n",
                " a debug: `Ready(1)`,\n",
                " b label: `b`,\n",
                " b debug: `Pending`",
            )
        );
    }

}

/// Assert two expressions are Ready(_) and their values are not equal.
///
/// * If true, return `()`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate assertables;
/// # use std::panic;
/// use std::task::Poll;
/// use std::task::Poll::*;
/// # fn main() {
/// let a: Poll<i8> = Ready(1);
/// let b: Poll<i8> = Ready(2);
/// assert_poll_ready_ne!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// let a: Poll<i8> = Ready(1);
/// let b: Poll<i8> = Ready(1);
/// assert_poll_ready_ne!(a, b);
/// # });
/// // assertion failed: `assert_poll_ready_ne!(a, b)`
/// // https://docs.rs/assertables/8.7.0/assertables/macro.assert_poll_ready_ne.html
/// //  a label: `a`,
/// //  a debug: `Ready(1)`,
/// //  a inner: `1`,
/// //  b label: `b`,
/// //  b debug: `Ready(1)`,
/// //  b inner: `1`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_poll_ready_ne!(a, b)`\n",
/// #     "https://docs.rs/assertables/8.7.0/assertables/macro.assert_poll_ready_ne.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `Ready(1)`,\n",
/// #     " a inner: `1`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `Ready(1)`,\n",
/// #     " b inner: `1`",
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_poll_ready_ne`](macro@crate::assert_poll_ready_ne)
/// * [`assert_poll_ready_ne_as_result`](macro@crate::assert_poll_ready_ne_as_result)
/// * [`debug_assert_poll_ready_ne`](macro@crate::debug_assert_poll_ready_ne)
///
#[deprecated(note = "Please rename from `assert_poll_ready_ne` to `assert_ready_ne` because more developers prefer the shorter name.")]
#[macro_export]
macro_rules! assert_poll_ready_ne {
    ($a:expr, $b:expr $(,)?) => ({
        match assert_poll_ready_ne_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($a:expr, $b:expr, $($message:tt)+) => ({
        match assert_poll_ready_ne_as_result!($a, $b) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert two expressions are Ready(_) and their values are not equal.
///
/// This macro provides the same statements as [`assert_poll_ready_ne`](macro.assert_poll_ready_ne.html),
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
/// * [`assert_poll_ready_ne`](macro@crate::assert_poll_ready_ne)
/// * [`assert_poll_ready_ne`](macro@crate::assert_poll_ready_ne)
/// * [`debug_assert_poll_ready_ne`](macro@crate::debug_assert_poll_ready_ne)
///
#[macro_export]
macro_rules! debug_assert_poll_ready_ne {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_poll_ready_ne!($($arg)*);
        }
    };
}
