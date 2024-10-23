//! Assert a ::std::io::Read read_to_string() value is less than another.
//!
//! Pseudocode:<br>
//! (a_reader.read_to_string(a_string) ⇒ a_string) < (b_reader.read_to_string(b_string) ⇒ b_string)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::io::Read;
//!
//! # fn main() {
//! let mut a = "alfa".as_bytes();
//! let mut b = "bravo".as_bytes();
//! assert_io_read_to_string_lt2!(a, b);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_io_read_to_string_lt2`](macro@crate::assert_io_read_to_string_lt2)
//! * [`assert_io_read_to_string_lt2_as_result`](macro@crate::assert_io_read_to_string_lt2_as_result)
//! * [`debug_assert_io_read_to_string_lt2`](macro@crate::debug_assert_io_read_to_string_lt2)

/// Assert a ::std::io::Read read_to_string() value is less than another.
///
/// Pseudocode:<br>
/// (a_reader.read_to_string(a_string) ⇒ a_string) < (b_reader.read_to_string(b_string) ⇒ b_string)
///
/// * If true, return Result `Ok((a_string, b_string))`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro provides the same statements as [`assert_`](macro.assert_.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_io_read_to_string_lt2`](macro@crate::assert_io_read_to_string_lt2)
/// * [`assert_io_read_to_string_lt2_as_result`](macro@crate::assert_io_read_to_string_lt2_as_result)
/// * [`debug_assert_io_read_to_string_lt2`](macro@crate::debug_assert_io_read_to_string_lt2)
///
#[macro_export]
macro_rules! assert_io_read_to_string_lt2_as_result {
    ($a_reader:expr, $b_reader:expr $(,)?) => {{
        let mut a_string = String::new();
        let mut b_string = String::new();
        match (
            $a_reader.read_to_string(&mut a_string),
            $b_reader.read_to_string(&mut b_string)
        ) {
            (Ok(_a_size), Ok(_b_size)) => {
                if a_string < b_string {
                    Ok((a_string, b_string))
                } else {
                    Err(
                        format!(
                            concat!(
                                "assertion failed: `assert_io_read_to_string_lt2!(a_reader, b_reader)`\n",
                                "https://docs.rs/assertables/9.0.0/assertables/macro.assert_io_read_to_string_lt2.html\n",
                                " a label: `{}`,\n",
                                " a debug: `{:?}`,\n",
                                " b label: `{}`,\n",
                                " b debug: `{:?}`,\n",
                                "       a: `{:?}`,\n",
                                "       b: `{:?}`"
                            ),
                            stringify!($a_reader),
                            $a_reader,
                            stringify!($b_reader),
                            $b_reader,
                            a_string,
                            b_string
                        )
                    )
                }
            },
            (a, b) => {
                Err(
                    format!(
                        concat!(
                            "assertion failed: `assert_io_read_to_string_lt2!(a_reader, b_reader)`\n",
                            "https://docs.rs/assertables/9.0.0/assertables/macro.assert_io_read_to_string_lt2.html\n",
                            "  a label: `{}`,\n",
                            "  a debug: `{:?}`,\n",
                            "  b label: `{}`,\n",
                            "  b debug: `{:?}`,\n",
                            "        a: `{:?}`,\n",
                            "        b: `{:?}`"
                        ),
                        stringify!($a_reader),
                        $a_reader,
                        stringify!($b_reader),
                        $b_reader,
                        a,
                        b
                    )
                )
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use std::io::Read;

    #[test]
    fn test_assert_io_read_to_string_lt_as_result_x_success() {
        let mut a = "alfa".as_bytes();
        let mut b = "bravo".as_bytes();
        let result = assert_io_read_to_string_lt2_as_result!(a, b);
        assert_eq!(
            result.unwrap(),
            (String::from("alfa"), String::from("bravo"))
        );
    }

    #[test]
    fn test_assert_io_read_to_string_lt_as_result_x_failure() {
        let mut a = "bravo".as_bytes();
        let mut b = "alfa".as_bytes();
        let result = assert_io_read_to_string_lt2_as_result!(a, b);
        assert_eq!(
            result.unwrap_err(),
            concat!(
                "assertion failed: `assert_io_read_to_string_lt2!(a_reader, b_reader)`\n",
                "https://docs.rs/assertables/9.0.0/assertables/macro.assert_io_read_to_string_lt2.html\n",
                " a label: `a`,\n",
                " a debug: `[]`,\n",
                " b label: `b`,\n",
                " b debug: `[]`,\n",
                "       a: `\"bravo\"`,\n",
                "       b: `\"alfa\"`"
            )
        );
    }
}

/// Assert a ::std::io::Read read_to_string() value is less than another.
///
/// Pseudocode:<br>
/// (a_reader.read_to_string(a_string) ⇒ a_string) < (b_reader.read_to_string(b_string) ⇒ b_string)
///
/// * If true, return `(a_string, b_string)`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// use assertables::*;
/// # use std::panic;
/// use std::io::Read;
///
/// # fn main() {
/// let mut a = "alfa".as_bytes();
/// let mut b = "bravo".as_bytes();
/// assert_io_read_to_string_lt2!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let mut a = "bravo".as_bytes();
/// let mut b = "alfa".as_bytes();
/// assert_io_read_to_string_lt2!(a, b);
/// # });
/// // assertion failed: `assert_io_read_to_string_lt2!(a_reader, b_reader)`
/// // https://docs.rs/assertables/9.0.0/assertables/macro.assert_io_read_to_string_lt2.html
/// //  a label: `a`,
/// //  a debug: `[]`,
/// //  b label: `b`,
/// //  b debug: `[]`,
/// //        a: `\"bravo\"`,
/// //        b: `\"alfa\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_io_read_to_string_lt2!(a_reader, b_reader)`\n",
/// #     "https://docs.rs/assertables/9.0.0/assertables/macro.assert_io_read_to_string_lt2.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `[]`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `[]`,\n",
/// #     "       a: `\"bravo\"`,\n",
/// #     "       b: `\"alfa\"`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_io_read_to_string_lt2`](macro@crate::assert_io_read_to_string_lt2)
/// * [`assert_io_read_to_string_lt2_as_result`](macro@crate::assert_io_read_to_string_lt2_as_result)
/// * [`debug_assert_io_read_to_string_lt2`](macro@crate::debug_assert_io_read_to_string_lt2)
///
#[macro_export]
macro_rules! assert_io_read_to_string_lt2 {
    ($a_reader:expr, $b:expr $(,)?) => {{
        match $crate::assert_io_read_to_string_lt2_as_result!($a_reader, $b) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    }};
    ($a_reader:expr, $b:expr, $($message:tt)+) => {{
        match $crate::assert_io_read_to_string_lt2_as_result!($a_reader, $b) {
            Ok(x) => x,
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a ::std::io::Read read_to_string() value is less than another.
///
/// Pseudocode:<br>
/// (a_reader.read_to_string(a_string) ⇒ a_string) < (b_reader.read_to_string(b_string) ⇒ b_string)
///
/// This macro provides the same statements as [`assert_io_read_to_string_lt2`](macro.assert_io_read_to_string_lt2.html),
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
/// [`::std::debug_assert`](https://doc.rust-lang.org/std/macro.debug_assert.html).
///
/// # Module macros
///
/// * [`assert_io_read_to_string_lt2`](macro@crate::assert_io_read_to_string_lt2)
/// * [`assert_io_read_to_string_lt2`](macro@crate::assert_io_read_to_string_lt2)
/// * [`debug_assert_io_read_to_string_lt2`](macro@crate::debug_assert_io_read_to_string_lt2)
///
#[macro_export]
macro_rules! debug_assert_io_read_to_string_lt2 {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_io_read_to_string_lt2!($($arg)*);
        }
    };
}