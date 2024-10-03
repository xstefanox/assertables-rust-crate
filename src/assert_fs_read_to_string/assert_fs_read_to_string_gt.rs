//! Assert a std::fs::read_to_string(path) value is greater than another.
//!
//! Pseudocode:<br>
//! std::fs::read_to_string(path1) > std::fs::read_to_string(path2)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! # fn main() {
//! let a ="alfa.txt";
//! let b ="bravo.txt";
//! assert_fs_read_to_string_gt!(&b, &a);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_fs_read_to_string_gt`](macro@crate::assert_fs_read_to_string_gt)
//! * [`assert_fs_read_to_string_gt_as_result`](macro@crate::assert_fs_read_to_string_gt_as_result)
//! * [`debug_assert_fs_read_to_string_gt`](macro@crate::debug_assert_fs_read_to_string_gt)

/// Assert a std::fs::read_to_string(path) value is greater than another.
///
/// Pseudocode:<br>
/// std::fs::read_to_string(path1) > std::fs::read_to_string(path2)
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_fs_read_to_string_gt`](macro.assert_fs_read_to_string_gt.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_gt`](macro@crate::assert_fs_read_to_string_gt)
/// * [`assert_fs_read_to_string_gt_as_result`](macro@crate::assert_fs_read_to_string_gt_as_result)
/// * [`debug_assert_fs_read_to_string_gt`](macro@crate::debug_assert_fs_read_to_string_gt)
///
#[macro_export]
macro_rules! assert_fs_read_to_string_gt_as_result {
    ($a_path:expr, $b_path:expr $(,)?) => {{
        match (&$a_path, &$b_path) {
            (a_path, b_path) => {
                match (std::fs::read_to_string(a_path), std::fs::read_to_string(b_path)) {
                    (Ok(a_string), Ok(b_string)) => {
                        if a_string > b_string {
                            Ok(())
                        } else {
                            Err(format!(
                                concat!(
                                    "assertion failed: `assert_fs_read_to_string_gt!(a_path, b_path)`\n",
                                    "https://docs.rs/assertables/8.13.0/assertables/macro.assert_fs_read_to_string_gt.html\n",
                                    " a_path label: `{}`,\n",
                                    " a_path debug: `{:?}`,\n",
                                    " b_path label: `{}`,\n",
                                    " b_path debug: `{:?}`,\n",
                                    "     a string: `{:?}`,\n",
                                    "     b string: `{:?}`"
                                ),
                                stringify!($a_path),
                                a_path,
                                stringify!($b_path),
                                b_path,
                                a_string,
                                b_string
                            ))
                        }
                    },
                    (a_result, b_result) => {
                        Err(format!(
                            concat!(
                                "assertion failed: `assert_fs_read_to_string_gt!(a_path, b_path)`\n",
                                "https://docs.rs/assertables/8.13.0/assertables/macro.assert_fs_read_to_string_gt.html\n",
                                " a_path label: `{}`,\n",
                                " a_path debug: `{:?}`,\n",
                                " b_path label: `{}`,\n",
                                " b_path debug: `{:?}`,\n",
                                "     a result: `{:?}`,\n",
                                "     b result: `{:?}`"
                            ),
                            stringify!($a_path),
                            a_path,
                            stringify!($b_path),
                            b_path,
                            a_result,
                            b_result
                        ))
                    }
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;
    #[allow(unused_imports)]
    use std::io::Read;
    use std::path::PathBuf;

    pub static DIR: Lazy<PathBuf> = Lazy::new(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("src")
            .join("std")
            .join("fs")
    });

    #[test]
    fn test_read_to_string_gt_as_result_x_success() {
        let a = DIR.join("bravo.txt");
        let b = DIR.join("alfa.txt");
        let result = assert_fs_read_to_string_gt_as_result!(&a, &b);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_read_to_string_gt_as_result_x_failure() {
        let a = DIR.join("alfa.txt");
        let b = DIR.join("bravo.txt");
        let result = assert_fs_read_to_string_gt_as_result!(&a, &b);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            format!(
                concat!(
                    "assertion failed: `assert_fs_read_to_string_gt!(a_path, b_path)`\n",
                    "https://docs.rs/assertables/8.13.0/assertables/macro.assert_fs_read_to_string_gt.html\n",
                    " a_path label: `&a`,\n",
                    " a_path debug: `{:?}`,\n",
                    " b_path label: `&b`,\n",
                    " b_path debug: `{:?}`,\n",
                    "     a string: `\"alfa\\n\"`,\n",
                    "     b string: `\"bravo\\n\"`"
                ),
                a,
                b
            )
        );
    }
}

/// Assert a std::fs::read_to_string(path) value is greater than another.
///
/// Pseudocode:<br>
/// std::fs::read_to_string(path1) > std::fs::read_to_string(path2)
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
/// use std::io::Read;
///
/// # fn main() {
/// let a = "alfa.txt";
/// let b = "bravo.txt";
/// assert_fs_read_to_string_gt!(&b, &a);
///
/// # let result = panic::catch_unwind(|| {
/// let a = "alfa.txt";
/// let b = "bravo.txt";
/// assert_fs_read_to_string_gt!(&a, &b);
/// # });
/// // assertion failed: `assert_fs_read_to_string_gt!(a_path, b_path)`
/// // https://docs.rs/assertables/8.13.0/assertables/macro.assert_fs_read_to_string_gt.html
/// //  a_path label: `&a`,
/// //  a_path debug: `\"alfa.txt\"`,
/// //  b_path label: `&b`,
/// //  b_path debug: `\"bravo.txt\"`,
/// //      a string: `\"alfa\\n\"`,
/// //      b string: `\"bravo\\n\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_fs_read_to_string_gt!(a_path, b_path)`\n",
/// #     "https://docs.rs/assertables/8.13.0/assertables/macro.assert_fs_read_to_string_gt.html\n",
/// #     " a_path label: `&a`,\n",
/// #     " a_path debug: `\"alfa.txt\"`,\n",
/// #     " b_path label: `&b`,\n",
/// #     " b_path debug: `\"bravo.txt\"`,\n",
/// #     "     a string: `\"alfa\\n\"`,\n",
/// #     "     b string: `\"bravo\\n\"`",
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_gt`](macro@crate::assert_fs_read_to_string_gt)
/// * [`assert_fs_read_to_string_gt_as_result`](macro@crate::assert_fs_read_to_string_gt_as_result)
/// * [`debug_assert_fs_read_to_string_gt`](macro@crate::debug_assert_fs_read_to_string_gt)
///
#[macro_export]
macro_rules! assert_fs_read_to_string_gt {
    ($a_path:expr, $b_path:expr $(,)?) => {{
        match $crate::assert_fs_read_to_string_gt_as_result!($a_path, $b_path) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    }};
    ($a_path:expr, $b_path:expr, $($message:tt)+) => {{
        match $crate::assert_fs_read_to_string_gt_as_result!($a_path, $b_path) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a std::fs::read_to_string(path) value is greater than another.
///
/// Pseudocode:<br>
/// std::fs::read_to_string(path1) > std::fs::read_to_string(path2)
///
/// This macro provides the same statements as [`assert_fs_read_to_string_gt`](macro.assert_fs_read_to_string_gt.html),
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
/// * [`assert_fs_read_to_string_gt`](macro@crate::assert_fs_read_to_string_gt)
/// * [`assert_fs_read_to_string_gt`](macro@crate::assert_fs_read_to_string_gt)
/// * [`debug_assert_fs_read_to_string_gt`](macro@crate::debug_assert_fs_read_to_string_gt)
///
#[macro_export]
macro_rules! debug_assert_fs_read_to_string_gt {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::std::fs::read_to_string_gt!($($arg)*);
        }
    };
}
