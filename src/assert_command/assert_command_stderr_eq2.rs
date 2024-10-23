//! Assert a command stderr string is equal to another.
//!
//! Pseudocode:<br>
//! (command1 ⇒ stderr) = (command2 ⇒ stderr)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::process::Command;
//!
//! # fn main() {
//! let mut a = Command::new("bin/printf-stderr");
//! a.args(["%s", "alfa"]);
//! let mut b = Command::new("bin/printf-stderr");
//! b.args(["%s", "alfa"]);
//! assert_command_stderr_eq2!(a, b);
//! # }
//! ```
//!
//! # Module macros
//!
//! * [`assert_command_stderr_eq2`](macro@crate::assert_command_stderr_eq2)
//! * [`assert_command_stderr_eq2_as_result`](macro@crate::assert_command_stderr_eq2_as_result)
//! * [`debug_assert_command_stderr_eq2`](macro@crate::debug_assert_command_stderr_eq2)

/// Assert a command stderr string is equal to another.
///
/// Pseudocode:<br>
/// (command1 ⇒ stderr) = (command2 ⇒ stderr)
///
/// * If true, return Result `Ok(stderr)`.
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
/// * [`assert_command_stderr_eq2`](macro@crate::assert_command_stderr_eq2)
/// * [`assert_command_stderr_eq2_as_result`](macro@crate::assert_command_stderr_eq2_as_result)
/// * [`debug_assert_command_stderr_eq2`](macro@crate::debug_assert_command_stderr_eq2)
///
#[macro_export]
macro_rules! assert_command_stderr_eq2_as_result {
    ($a_command:expr, $b_command:expr $(,)?) => {{
        match ($a_command.output(), $b_command.output()) {
            (Ok(a), Ok(b)) => {
                let a = a.stderr;
                let b = b.stderr;
                if a.eq(&b) {
                    Ok((a, b))
                } else {
                    Err(
                        format!(
                            concat!(
                                "assertion failed: `assert_command_stderr_eq2!(a_command, b_command)`\n",
                                "https://docs.rs/assertables/9.0.0/assertables/macro.assert_command_stderr_eq2.html\n",
                                " a label: `{}`,\n",
                                " a debug: `{:?}`,\n",
                                " b label: `{}`,\n",
                                " b debug: `{:?}`,\n",
                                "       a: `{:?}`,\n",
                                "       b: `{:?}`"
                            ),
                            stringify!($a_command),
                            $a_command,
                            stringify!($b_command),
                            $b_command,
                            a,
                            b
                        )
                    )
                }
            },
            (a, b) => {
                Err(
                    format!(
                        concat!(
                            "assertion failed: `assert_command_stderr_eq2!(a_command, b_command)`\n",
                            "https://docs.rs/assertables/9.0.0/assertables/macro.assert_command_stderr_eq2.html\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`,\n",
                            "       a: `{:?}`,\n",
                            "       b: `{:?}`"
                        ),
                        stringify!($a_command),
                        $a_command,
                        stringify!($b_command),
                        $b_command,
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

    use std::process::Command;

    #[test]
    fn eq() {
        let mut a = Command::new("bin/printf-stderr");
        a.args(["%s", "alfa"]);
        let mut b = Command::new("bin/printf-stderr");
        b.args(["%s", "alfa"]);
        let result = assert_command_stderr_eq2_as_result!(a, b);
        assert_eq!(
            result.unwrap(),
            (vec![b'a', b'l', b'f', b'a'], vec![b'a', b'l', b'f', b'a'])
        );
    }

    #[test]
    fn lt() {
        let mut a = Command::new("bin/printf-stderr");
        a.args(["%s", "alfa"]);
        let mut b = Command::new("bin/printf-stderr");
        b.args(["%s", "zz"]);
        let result = assert_command_stderr_eq2_as_result!(a, b);
        let actual = result.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_command_stderr_eq2!(a_command, b_command)`\n",
            "https://docs.rs/assertables/9.0.0/assertables/macro.assert_command_stderr_eq2.html\n",
            " a label: `a`,\n",
            " a debug: `\"bin/printf-stderr\" \"%s\" \"alfa\"`,\n",
            " b label: `b`,\n",
            " b debug: `\"bin/printf-stderr\" \"%s\" \"zz\"`,\n",
            "       a: `[97, 108, 102, 97]`,\n",
            "       b: `[122, 122]`"
        );
        assert_eq!(actual, expect);
    }

    #[test]
    fn gt() {
        let mut a = Command::new("bin/printf-stderr");
        a.args(["%s", "alfa"]);
        let mut b = Command::new("bin/printf-stderr");
        b.args(["%s", "aa"]);
        let result = assert_command_stderr_eq2_as_result!(a, b);
        let actual = result.unwrap_err();
        let expect = concat!(
            "assertion failed: `assert_command_stderr_eq2!(a_command, b_command)`\n",
            "https://docs.rs/assertables/9.0.0/assertables/macro.assert_command_stderr_eq2.html\n",
            " a label: `a`,\n",
            " a debug: `\"bin/printf-stderr\" \"%s\" \"alfa\"`,\n",
            " b label: `b`,\n",
            " b debug: `\"bin/printf-stderr\" \"%s\" \"aa\"`,\n",
            "       a: `[97, 108, 102, 97]`,\n",
            "       b: `[97, 97]`"
        );
        assert_eq!(actual, expect);
    }
}

/// Assert a command stderr string is equal to another.
///
/// Pseudocode:<br>
/// (command1 ⇒ stderr) = (command2 ⇒ stderr)
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
/// use std::process::Command;
///
/// # fn main() {
/// let mut a = Command::new("bin/printf-stderr");
/// a.args(["%s", "alfa"]);
/// let mut b = Command::new("bin/printf-stderr");
/// b.args(["%s", "alfa"]);
/// assert_command_stderr_eq2!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let mut a = Command::new("bin/printf-stderr");
/// a.args(["%s", "alfa"]);
/// let mut b = Command::new("bin/printf-stderr");
/// b.args(["%s", "zz"]);
/// assert_command_stderr_eq2!(a, b);
/// # });
/// // assertion failed: `assert_command_stderr_eq2!(a_command, b_command)`
/// // https://docs.rs/assertables/9.0.0/assertables/macro.assert_command_stderr_eq2.html
/// //  a label: `a`,
/// //  a debug: `\"bin/printf-stderr\" \"%s\" \"alfa\"`,
/// //  b label: `b`,
/// //  b debug: `\"bin/printf-stderr\" \"%s\" \"zz\"`,
/// //        a: `[97, 108, 102, 97]`,
/// //        b: `[122, 122]`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let expect = concat!(
/// #     "assertion failed: `assert_command_stderr_eq2!(a_command, b_command)`\n",
/// #     "https://docs.rs/assertables/9.0.0/assertables/macro.assert_command_stderr_eq2.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `\"bin/printf-stderr\" \"%s\" \"alfa\"`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `\"bin/printf-stderr\" \"%s\" \"zz\"`,\n",
/// #     "       a: `[97, 108, 102, 97]`,\n",
/// #     "       b: `[122, 122]`"
/// # );
/// # assert_eq!(actual, expect);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_command_stderr_eq2`](macro@crate::assert_command_stderr_eq2)
/// * [`assert_command_stderr_eq2_as_result`](macro@crate::assert_command_stderr_eq2_as_result)
/// * [`debug_assert_command_stderr_eq2`](macro@crate::debug_assert_command_stderr_eq2)
///
#[macro_export]
macro_rules! assert_command_stderr_eq2 {
    ($a_command:expr, $b_command:expr $(,)?) => {{
        match $crate::assert_command_stderr_eq2_as_result!($a_command, $b_command) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    }};
    ($a_command:expr, $b_command:expr, $($message:tt)+) => {{
        match $crate::assert_command_stderr_eq2_as_result!($a_command, $b_command) {
            Ok(x) => x,
            Err(_err) => panic!("{}", $($message)+),
        }
    }};
}

/// Assert a command stderr string is equal to another.
///
/// This macro provides the same statements as [`assert_command_stderr_eq2 {`](macro.assert_command_stderr_eq2 {.html),
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
/// * [`assert_command_stderr_eq2 {`](macro@crate::assert_command_stderr_eq2 {)
/// * [`assert_command_stderr_eq2 {`](macro@crate::assert_command_stderr_eq2 {)
/// * [`debug_assert_command_stderr_eq2 {`](macro@crate::debug_assert_command_stderr_eq2 {)
///
#[macro_export]
macro_rules! debug_assert_command_stderr_eq2 {
    ($($arg:tt)*) => {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_command_stderr_eq2!($($arg)*);
        }
    };
}