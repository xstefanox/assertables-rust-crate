//! Assert a std::fs::read_to_string() is a match to a regex.
//!
//! # Example
//!
//! ```rust
//! # #[macro_use] extern crate assertables;
//! use regex::Regex;
//!
//! # fn main() {
//! let path = "alfa.txt";
//! let matcher = Regex::new(r"alfa").unwrap();
//! assert_fs_read_to_string_matches!(&path, &matcher);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_fs_read_to_string_matches`](macro@crate::assert_fs_read_to_string_matches)
//! * [`assert_fs_read_to_string_matches_as_result`](macro@crate::assert_fs_read_to_string_matches_as_result)
//! * [`debug_assert_fs_read_to_string_matches`](macro@crate::debug_assert_fs_read_to_string_matches)

/// Assert a std::fs::read_to_string() is a match to a regex.
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err` with a diagnostic message.
///
/// This macro provides the same statements as [`assert_fs_read_to_string_matches`](macro.assert_fs_read_to_string_matches.html),
/// except this macro returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_matches`](macro@crate::assert_fs_read_to_string_matches)
/// * [`assert_fs_read_to_string_matches_as_result`](macro@crate::assert_fs_read_to_string_matches_as_result)
/// * [`debug_assert_fs_read_to_string_matches`](macro@crate::debug_assert_fs_read_to_string_matches)
///
#[macro_export]
macro_rules! assert_fs_read_to_string_matches_as_result {
    ($path:expr, $matcher:expr $(,)?) => ({
        match (&$path, &$matcher) {
            (path, matcher) => {
                let read_result = ::std::fs::read_to_string(path);
                if let Err(read_err) = read_result {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_fs_read_to_string_matches!(path, matcher)`\n",
                            "https://docs.rs/assertables/8.7.0/assertables/macro.assert_fs_read_to_string_matches.html\n",
                            "    path label: `{}`,\n",
                            "    path debug: `{:?}`,\n",
                            " matcher label: `{}`,\n",
                            " matcher debug: `{:?}`,\n",
                            "      read err: `{:?}`"
                        ),
                        stringify!($path),
                        path,
                        stringify!($matcher),
                        matcher,
                        read_err
                    ))
                } else {
                    let read_string = read_result.unwrap();
                    if matcher.is_match(read_string.as_str()) {
                        Ok(())
                    } else {
                        Err(format!(
                            concat!(
                                "assertion failed: `assert_fs_read_to_string_matches!(path, matcher)`\n",
                                "https://docs.rs/assertables/8.7.0/assertables/macro.assert_fs_read_to_string_matches.html\n",
                                "    path label: `{}`,\n",
                                "    path debug: `{:?}`,\n",
                                " matcher label: `{}`,\n",
                                " matcher debug: `{:?}`,\n",
                                "   read string: `{:?}`",
                            ),
                            stringify!($path),
                            path,
                            stringify!($matcher),
                            matcher,
                            read_string
                        ))
                    }
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;
    use regex::Regex;
    use std::path::PathBuf;

    pub static DIR: Lazy<PathBuf> = Lazy::new(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("src")
            .join("std")
            .join("fs")
    });

    #[test]
    fn test_read_to_string_matches_as_result_x_success() {
        let path = DIR.join("alfa.txt");
        let matcher = Regex::new(r"alfa").unwrap();
        let result = assert_fs_read_to_string_matches_as_result!(&path, &matcher);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_read_to_string_matches_as_result_x_failure() {
        let path = DIR.join("alfa.txt");
        let matcher = Regex::new(r"zzz").unwrap();
        let result = assert_fs_read_to_string_matches_as_result!(&path, &matcher);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            format!(
                concat!(
                    "assertion failed: `assert_fs_read_to_string_matches!(path, matcher)`\n",
                    "https://docs.rs/assertables/8.7.0/assertables/macro.assert_fs_read_to_string_matches.html\n",
                    "    path label: `&path`,\n",
                    "    path debug: `{:?}`,\n",
                    " matcher label: `&matcher`,\n",
                    " matcher debug: `Regex(\"zzz\")`,\n",
                    "   read string: `\"alfa\\n\"`",
                ),
                path
            )
        );
    }
}

/// Assert a std::fs::read_to_string() is a match to a regex.
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
/// use std::io::Read;
/// use regex::Regex;
///
/// # fn main() {
/// let path = "alfa.txt";
/// let matcher = Regex::new(r"alfa").unwrap();
/// assert_fs_read_to_string_matches!(&path, &matcher);
///
/// # let result = panic::catch_unwind(|| {
/// let path = "alfa.txt";
/// let matcher = Regex::new(r"zzz").unwrap();
/// assert_fs_read_to_string_matches!(&path, &matcher);
/// # });
/// // assertion failed: `assert_fs_read_to_string_matches!(path, matcher)`
/// // https://docs.rs/assertables/8.7.0/assertables/macro.assert_fs_read_to_string_matches.html
/// //     path label: `&path`,
/// //     path debug: `\"alfa.txt\"`,
/// //  matcher label: `&matcher`,
/// //  matcher debug: `Regex(\"zzz\")`,
/// //    read string: `\"alfa\\n\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_fs_read_to_string_matches!(path, matcher)`\n",
/// #     "https://docs.rs/assertables/8.7.0/assertables/macro.assert_fs_read_to_string_matches.html\n",
/// #     "    path label: `&path`,\n",
/// #     "    path debug: `\"alfa.txt\"`,\n",
/// #     " matcher label: `&matcher`,\n",
/// #     " matcher debug: `Regex(\"zzz\")`,\n",
/// #     "   read string: `\"alfa\\n\"`",
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_matches`](macro@crate::assert_fs_read_to_string_matches)
/// * [`assert_fs_read_to_string_matches_as_result`](macro@crate::assert_fs_read_to_string_matches_as_result)
/// * [`debug_assert_fs_read_to_string_matches`](macro@crate::debug_assert_fs_read_to_string_matches)
///
#[macro_export]
macro_rules! assert_fs_read_to_string_matches {
    ($path:expr, $matcher:expr $(,)?) => ({
        match assert_fs_read_to_string_matches_as_result!($path, $matcher) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    });
    ($path:expr, $matcher:expr, $($message:tt)+) => ({
        match assert_fs_read_to_string_matches_as_result!($path, $matcher) {
            Ok(()) => (),
            Err(_err) => panic!("{}", $($message)+),
        }
    });
}

/// Assert a std::fs::read_to_string() is a match to a regex.
///
/// This macro provides the same statements as [`assert_fs_read_to_string_matches`](macro.assert_fs_read_to_string_matches.html),
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
/// * [`assert_fs_read_to_string_matches`](macro@crate::assert_fs_read_to_string_matches)
/// * [`assert_fs_read_to_string_matches`](macro@crate::assert_fs_read_to_string_matches)
/// * [`debug_assert_fs_read_to_string_matches`](macro@crate::debug_assert_fs_read_to_string_matches)
///
#[macro_export]
macro_rules! debug_assert_fs_read_to_string_matches {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::std::fs::read_to_string_matches!($($arg)*);
        }
    };
}
